//! Field arithmetic modulo \\(p = 2\^{255} - 19\\), using \\(32\\)-bit
//! limbs with \\(64\\)-bit products.

use core::fmt::Debug;
use core::ops::Neg;
use core::ops::{Add, AddAssign};
use core::ops::{Mul, MulAssign};
use core::ops::{Sub, SubAssign};

use elliptic_curve::{
    bigint::{risc0, Encoding, U256},
    subtle::{Choice, ConditionallySelectable},
};

#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

/// A `FieldElementR0` represents an element of the field
/// \\( \mathbb Z / (2\^{255} - 19)\\).
///
/// # Note
///
/// The `curve25519_dalek::field` module provides a type alias
/// `curve25519_dalek::field::FieldElement` to either `FieldElement51`
/// or `FieldElement2625`.
///
/// The backend-specific type `FieldElement2625` should not be used
/// outside of the `curve25519_dalek::field` module.

/// prime 2^255 - 19 which defines the field.
const P: U256 =
    U256::from_be_hex("7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED");

#[derive(Copy, Clone)]
pub struct FieldElementR0(pub(crate) U256);

impl Debug for FieldElementR0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
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
    fn add_assign(&mut self, _rhs: &'b FieldElementR0) {
        let result = self.0.add_mod(&_rhs.0, &P);
        self.0 = result
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
        let result = self.0.sub_mod(&_rhs.0, &P);
        self.0 = result
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
        let result = risc0::modmul_u256(&self.0, &_rhs.0, &P);
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

    const LOW_255_BITS: U256 =
        U256::from_be_hex("7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");

    /// Invert the sign of this field element
    pub fn negate(&mut self) {
        let result = risc0::modmul_u256(&self.0, &Self::MINUS_ONE.0, &P);
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
        let val: U256 = U256::from_le_bytes(*data);
        let val = val.bitand(&Self::LOW_255_BITS);
        let val = risc0::modmul_u256(&val, &FieldElementR0::ONE.0, &P);
        FieldElementR0(val)
    }

    /// Serialize this `FieldElementR0` to a 32-byte array.  The
    /// encoding is canonical.
    #[allow(clippy::identity_op)]
    pub fn as_bytes(&self) -> [u8; 32] {
        self.0.to_le_bytes()
    }

    /// Compute `self^2`.
    pub fn square(&self) -> FieldElementR0 {
        let result = risc0::modmul_u256(&self.0, &self.0, &P);
        FieldElementR0(result)
    }

    /// Compute `2*self^2`.
    pub fn square2(&self) -> FieldElementR0 {
        let squared = self.square();
        let result = risc0::modmul_u256(&Self::TWO.0, &squared.0, &P);
        FieldElementR0(result)
    }
}
