//! Arithmetic mod 2^252 + 27742317777372353535851937790883648493
//! with RISC0 Acceleration

use core::fmt::Debug;
use crypto_bigint::{risc0, Encoding, U256};

#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

use crate::constants;

/// Multiplicative Inverse of R mod L where R is the Montgomery modulus 2^261
const R_INVERSE: U256 =
    U256::from_be_hex("064EDB637937F48C1B0A73AA1C7FD1B5FD934BE6D1D6D67AC7421B8F04C727E2");

/// 2^256 mod L
const TWO_POW_TWO_FIFTY_SIX: U256 =
    U256::from_be_hex("0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEC6EF5BF4737DCF70D6EC31748D98951D");

/// The `ScalarR0` struct represents an element in \\(\mathbb{Z} / \ell\mathbb{Z}\\)
#[derive(Copy, Clone)]
pub struct ScalarR0(pub U256);

impl Debug for ScalarR0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        write!(f, "ScalarR0: {:?}", &self.0)
    }
}

#[cfg(feature = "zeroize")]
impl Zeroize for ScalarR0 {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}

impl ScalarR0 {
    /// The scalar \\( -1 mod L \\).
    pub const MINUS_ONE: ScalarR0 = ScalarR0(U256::from_be_hex(
        "1000000000000000000000000000000014DEF9DEA2F79CD65812631A5CF5D3EC",
    ));

    /// Unpack a 32 byte / 256 bit scalar.
    pub fn from_bytes(bytes: &[u8; 32]) -> ScalarR0 {
        ScalarR0(U256::from_le_bytes(*bytes))
    }

    /// Reduce a 64 byte / 512 bit scalar mod l.
    pub fn from_bytes_wide(bytes: &[u8; 64]) -> ScalarR0 {
        let lo: U256 = U256::from_le_bytes(
            bytes[0..32]
                .try_into()
                .expect("unable to parse low 32 bytes"),
        );
        let hi: U256 = U256::from_le_bytes(
            bytes[32..]
                .try_into()
                .expect("unable to parse high 32 bytes"),
        );

        let hi_shifted_left_256 = risc0::modmul_u256(&hi, &TWO_POW_TWO_FIFTY_SIX, &constants::L.0);
        // add_mod assumes the lhs + rhs is less than 2p. To guarantee this, we need to mod
        // lo and hi by L
        let lo = risc0::modmul_u256(&lo, &U256::ONE, &constants::L.0);
        let total = hi_shifted_left_256.add_mod(&lo, &constants::L.0);

        ScalarR0(total)
    }

    /// Pack the limbs of this `ScalarR0` into 32 bytes.
    #[allow(clippy::identity_op)]
    pub fn as_bytes(&self) -> [u8; 32] {
        let val = risc0::modmul_u256(&self.0, &U256::ONE, &constants::L.0);
        val.to_le_bytes()
    }

    /// Compute `a + b` (mod l).
    pub fn add(a: &ScalarR0, b: &ScalarR0) -> ScalarR0 {
        let result = a.0.add_mod(&b.0, &constants::L.0);
        ScalarR0(result)
    }

    /// Compute `a - b` (mod l).
    pub fn sub(a: &ScalarR0, b: &ScalarR0) -> ScalarR0 {
        let result = a.0.sub_mod(&b.0, &constants::L.0);
        ScalarR0(result)
    }

    /// Compute `-1 * a` (mod l).
    pub fn negate(a: &ScalarR0) -> ScalarR0 {
        let result = risc0::modmul_u256(&a.0, &Self::MINUS_ONE.0, &constants::L.0);
        ScalarR0(result)
    }

    /// Compute `a` (mod l).
    pub fn reduce(a: &ScalarR0) -> ScalarR0 {
        let result = risc0::modmul_u256(&a.0, &U256::ONE, &constants::L.0);
        ScalarR0(result)
    }

    /// Compute `a * b` (mod l).
    #[inline(never)]
    pub fn mul(a: &ScalarR0, b: &ScalarR0) -> ScalarR0 {
        let ab = risc0::modmul_u256(&a.0, &b.0, &constants::L.0);
        ScalarR0(ab)
    }

    /// Compute `a^2` (mod l).
    #[inline(never)]
    #[allow(dead_code)] // XXX we don't expose square() via the Scalar API
    pub fn square(&self) -> ScalarR0 {
        let aa = risc0::modmul_u256(&self.0, &self.0, &constants::L.0);
        ScalarR0(aa)
    }

    /// Compute `(a * b) / R` (mod l), where R is the Montgomery modulus 2^261
    #[inline(never)]
    pub fn montgomery_mul(a: &ScalarR0, b: &ScalarR0) -> ScalarR0 {
        let ab = risc0::modmul_u256_denormalized(&a.0, &b.0, &constants::L.0);
        let ab_r_inverse = risc0::modmul_u256(&ab, &R_INVERSE, &constants::L.0);
        ScalarR0(ab_r_inverse)
    }

    /// Compute `(a^2) / R` (mod l) in Montgomery form, where R is the Montgomery modulus 2^261
    #[inline(never)]
    pub fn montgomery_square(&self) -> ScalarR0 {
        let squared = risc0::modmul_u256_denormalized(&self.0, &self.0, &constants::L.0);
        let squared_r_inverse = risc0::modmul_u256(&squared, &R_INVERSE, &constants::L.0);
        ScalarR0(squared_r_inverse)
    }

    /// Puts a ScalarR0 in to Montgomery form, i.e. computes `a*R (mod l)`
    #[inline(never)]
    pub fn as_montgomery(&self) -> ScalarR0 {
        let result = risc0::modmul_u256(&self.0, &constants::R.0, &constants::L.0);
        ScalarR0(result)
    }

    /// Takes a ScalarR0 out of Montgomery form, i.e. computes `a/R (mod l)`
    #[allow(clippy::wrong_self_convention)]
    pub fn from_montgomery(&self) -> ScalarR0 {
        let a_r_inverse = risc0::modmul_u256(&self.0, &R_INVERSE, &constants::L.0);
        ScalarR0(a_r_inverse)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Note: x is 2^253-1 which is slightly larger than the largest scalar produced by
    /// this implementation (l-1), and should verify there are no overflows for valid scalars
    ///
    /// x = 2^253-1 = 14474011154664524427946373126085988481658748083205070504932198000989141204991
    /// x = 7237005577332262213973186563042994240801631723825162898930247062703686954002 mod l
    /// x = 5147078182513738803124273553712992179887200054963030844803268920753008712037*R mod l in Montgomery form
    pub static X: ScalarR0 = ScalarR0(U256::from_be_hex(
        "0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEB2106215D086329A7ED9CE5A30A2C12",
    ));

    /// x^2 = 3078544782642840487852506753550082162405942681916160040940637093560259278169 mod l
    pub static XX: ScalarR0 = ScalarR0(U256::from_be_hex(
        "06CE65046DF0C268F73BB1CF485FD6F9F38A31531640FFD0EC01668020217559",
    ));

    /// x^2 = 2912514428060642753613814151688322857484807845836623976981729207238463947987*R mod l in Montgomery form
    pub static XX_MONT: ScalarR0 = ScalarR0(U256::from_be_hex(
        "030EDB637937F48C1B0A73AA1C7FD1B5F92C4331DB769B6590AE3AA7752B4D2E",
    ));

    /// y = 6145104759870991071742105800796537629880401874866217824609283457819451087098
    pub static Y: ScalarR0 = ScalarR0(U256::from_be_hex(
        "0D96018BB8255FFFCC11FAD13433D2BAF0672BBF9D75E1ECDACB75071E1458FA",
    ));

    /// x*y = 36752150652102274958925982391442301741
    pub static XY: ScalarR0 = ScalarR0(U256::from_be_hex(
        "000000000000000000000000000000001BA634ED50D71D84E02EE6D76BA7632D",
    ));

    /// x*y = 3783114862749659543382438697751927473898937741870308063443170013240655651591*R mod l in Montgomery form
    pub static XY_MONT: ScalarR0 = ScalarR0(U256::from_be_hex(
        "0BDC1CE001340933F1F248B4F7900DE0430C69094F0A867BD78C9C23277B51E1",
    ));

    /// a = 2351415481556538453565687241199399922945659411799870114962672658845158063753
    pub static A: ScalarR0 = ScalarR0(U256::from_be_hex(
        "0532DA9FAB8C6B3F6E4FEC4C4A4AA782AAE3EE1BC3D2A67C0C45236C07B3BE89",
    ));

    /// b = 4885590095775723760407499321843594317911456947580037491039278279440296187236
    pub static B: ScalarR0 = ScalarR0(U256::from_be_hex(
        "0ACD2560547394C091B013B3B5B5587D69FB0BC2DF24F65A4BCD3FAE55421564",
    ));

    /// a+b = 0
    /// a-b = 4702830963113076907131374482398799845891318823599740229925345317690316127506
    pub static AB: ScalarR0 = ScalarR0(U256::from_be_hex(
        "0A65B53F5718D67EDC9FD89894954F0555C7DC3787A54CF8188A46D80F677D12",
    ));

    // c = (2^512 - 1) % l = 1627715501170711445284395025044413883736156588369414752970002579683115011840
    pub static C: ScalarR0 = ScalarR0(U256::from_be_hex(
        "0399411B7C309A3DCEEC73D217F5BE65D00E1BA768859347A40611E3449C0F00",
    ));

    #[test]
    fn mul_max() {
        let res = ScalarR0::mul(&X, &X);
        assert!(res.0 == XX.0);
    }

    #[test]
    fn square_max() {
        let res = X.square();
        assert!(res.0 == XX.0);
    }

    #[test]
    fn montgomery_mul_max() {
        let res = ScalarR0::montgomery_mul(&X, &X);
        assert!(res.0 == XX_MONT.0);
    }

    #[test]
    fn montgomery_square_max() {
        let res = X.montgomery_square();
        assert!(res.0 == XX_MONT.0);
    }

    #[test]
    fn mul() {
        let res = ScalarR0::mul(&X, &Y);
        assert!(res.0 == XY.0);
    }

    #[test]
    fn montgomery_mul() {
        let res = ScalarR0::montgomery_mul(&X, &Y);
        assert!(res.0 == XY_MONT.0);
    }

    #[test]
    fn add() {
        let res = ScalarR0::add(&A, &B);
        let zero = ScalarR0(U256::ZERO);
        assert!(res.0 == zero.0);
    }

    #[test]
    fn sub() {
        let res = ScalarR0::sub(&A, &B);
        assert!(res.0 == AB.0);
    }

    #[test]
    fn from_bytes_wide() {
        let bignum = [255u8; 64]; // 2^512 - 1
        let reduced = ScalarR0::from_bytes_wide(&bignum);
        assert!(reduced.0 == C.0);
    }
}
