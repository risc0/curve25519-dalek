//! Field arithmetic modulo \\(p = 2\^{255} - 19\\), using \\(32\\)-bit
//! limbs

use core::fmt::Debug;
use core::ops::Neg;
use core::ops::{Add, AddAssign};
use core::ops::{Mul, MulAssign};
use core::ops::{Sub, SubAssign};

use crypto_bigint::{risc0, Limb, Encoding, U256};
use subtle::{Choice, ConditionallySelectable, ConstantTimeLess};

#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

/// A `FieldElementR0` represents an element of the field
/// \\( \mathbb Z / (2\^{255} - 19)\\). `FieldElementR0`
/// leverages RISC Zero's big integer accelerated zkvm circuit.
///
/// # Note
///
/// The `curve25519_dalek::field` module provides a type alias
/// `curve25519_dalek::field::FieldElement` to either `FieldElement51`,
/// `FieldElement2625` or `FieldElementR0`.
///
/// The backend-specific type `FieldElementR0` should not be used
/// outside of the `curve25519_dalek::field` module.

/// prime 2^255 - 19 which defines the field.
const P: U256 =
    U256::from_be_hex("7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED");

#[derive(Copy, Clone)]
pub struct FieldElementR0(pub(crate) U256);

impl Debug for FieldElementR0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "FieldElementR0(U256::from_be_hex({:?}))",
            hex::encode(&self.0.to_be_bytes())
        )
    }
}

#[cfg(feature = "zeroize")]
impl Zeroize for FieldElementR0 {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}

impl<'b> AddAssign<&'b FieldElementR0> for FieldElementR0 {
    fn add_assign(&mut self, rhs: &'b FieldElementR0) {
        let self_limbs = self.0.as_limbs();
        let rhs_limbs = rhs.0.as_limbs();

        // Carrying addition of self and rhs, with the overflow correction added in.
        // Correction is added to carries with wrapping_add since they cannot overflow.
        let (a0, carry0) = self_limbs[0].adc(rhs_limbs[0], Limb::ZERO);
        let (a1, carry1) =
            self_limbs[1].adc(rhs_limbs[1], carry0);
        let (a2, carry2) =
            self_limbs[2].adc(rhs_limbs[2], carry1);
        let (a3, carry3) =
            self_limbs[3].adc(rhs_limbs[3], carry2);
        let (a4, carry4) =
            self_limbs[4].adc(rhs_limbs[4], carry3);
        let (a5, carry5) =
            self_limbs[5].adc(rhs_limbs[5], carry4);
        let (a6, carry6) =
            self_limbs[6].adc(rhs_limbs[6], carry5);
        let (a7, carry7) =
            self_limbs[7].adc(rhs_limbs[7], carry6);
        self.0 = U256::from([a0, a1, a2, a3, a4, a5, a6, a7]);

        // a cooperative prover can make sure that overflow can never happen
        assert!(carry7.0 == 0);

        self.0 = risc0::modmul_u256_denormalized(&self.0, &U256::ONE, &P);
    }
}

impl<'a, 'b> Add<&'b FieldElementR0> for &'a FieldElementR0 {
    type Output = FieldElementR0;
    fn add(self, _rhs: &'b FieldElementR0) -> FieldElementR0 {
        let mut output = *self;
        output += _rhs;
        output
    }
}

impl<'b> SubAssign<&'b FieldElementR0> for FieldElementR0 {
    fn sub_assign(&mut self, _rhs: &'b FieldElementR0) {
        self.add_assign(&_rhs.neg());
    }
}

impl<'a, 'b> Sub<&'b FieldElementR0> for &'a FieldElementR0 {
    type Output = FieldElementR0;
    fn sub(self, _rhs: &'b FieldElementR0) -> FieldElementR0 {
        let mut output = *self;
        output -= _rhs;
        output
    }
}

impl<'b> MulAssign<&'b FieldElementR0> for FieldElementR0 {
    fn mul_assign(&mut self, _rhs: &'b FieldElementR0) {
        let result = risc0::modmul_u256_denormalized(&self.0, &_rhs.0, &P);
        self.0 = result;
    }
}

impl<'a, 'b> Mul<&'b FieldElementR0> for &'a FieldElementR0 {
    type Output = FieldElementR0;
    fn mul(self, _rhs: &'b FieldElementR0) -> FieldElementR0 {
        let mut output = *self;
        output *= _rhs;
        output
    }
}

impl<'a> Neg for &'a FieldElementR0 {
    type Output = FieldElementR0;
    fn neg(self) -> FieldElementR0 {
        let mut output = *self;
        output.negate();
        output
    }
}

impl ConditionallySelectable for FieldElementR0 {
    fn conditional_select(
        a: &FieldElementR0,
        b: &FieldElementR0,
        choice: Choice,
    ) -> FieldElementR0 {
        FieldElementR0(U256::conditional_select(&a.0, &b.0, choice))
    }
}

impl FieldElementR0 {
    /// The scalar \\( 0 \\).
    pub const ZERO: FieldElementR0 = FieldElementR0(U256::ZERO);
    /// The scalar \\( 1 \\).
    pub const ONE: FieldElementR0 = FieldElementR0(U256::ONE);
    /// The scalar \\( 2 \\).
    pub const TWO: FieldElementR0 = FieldElementR0(U256::from_be_hex(
        "0000000000000000000000000000000000000000000000000000000000000002",
    ));
    /// The scalar \\( -1 \\). Set to P - 1
    pub const MINUS_ONE: FieldElementR0 = FieldElementR0(U256::from_be_hex(
        "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEC",
    ));

    /// Invert the sign of this field element
    pub fn negate(&mut self) {
        let result = risc0::modmul_u256_denormalized(&self.0, &Self::MINUS_ONE.0, &P);
        self.0 = result;
    }

    /// Given `k > 0`, return `self^(2^k)`.
    pub fn pow2k(&self, k: u32) -> FieldElementR0 {
        debug_assert!(k > 0);
        let mut z = self.square();
        for _ in 1..k {
            z = z.square();
        }
        z
    }

    /// Load a `FieldElementR0` from the low 255 bits of a 256-bit
    /// input.
    pub fn from_bytes(data: &[u8; 32]) -> FieldElementR0 {
        let mut val: U256 = U256::from_le_bytes(*data);
        let val_words = val.as_words_mut();
        val_words[7] = val_words[7] & 0x7FFFFFFF;
        let val = U256::from_words(*val_words);
        // Use a modular multiplication by one to reduce the value to [0, p).
        let val = risc0::modmul_u256_denormalized(&val, &FieldElementR0::ONE.0, &P);
        FieldElementR0(val)
    }

    /// Serialize this `FieldElementR0` to a 32-byte array.  The
    /// encoding is canonical.
    #[allow(clippy::identity_op)]
    pub fn as_bytes(&self) -> [u8; 32] {
        // Check that the output is normalized. This will always be the case if the host is
        // cooperative.
        assert!(self.0.ct_lt(&P).unwrap_u8() == 1);
        self.0.to_le_bytes()
    }

    /// Compute `self^2`.
    pub fn square(&self) -> FieldElementR0 {
        let result = risc0::modmul_u256_denormalized(&self.0, &self.0, &P);
        FieldElementR0(result)
    }

    /// Compute `2*self^2`.
    pub fn square2(&self) -> FieldElementR0 {
        let squared = self.square();
        let result = risc0::modmul_u256_denormalized(&Self::TWO.0, &squared.0, &P);
        FieldElementR0(result)
    }
}
