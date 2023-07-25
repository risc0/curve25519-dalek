use super::field::FieldElementR0;
use super::scalar::ScalarR0;
use crate::edwards::EdwardsPoint;

use elliptic_curve::bigint::U256;

/// The value of minus one, equal to `-&FieldElement::ONE`
pub(crate) const MINUS_ONE: FieldElementR0 = FieldElementR0::MINUS_ONE;

/// Edwards `d` value, equal to `-121665/121666 mod p`.
pub(crate) const EDWARDS_D: FieldElementR0 = FieldElementR0(U256::from_be_hex(
    "52036CEE2B6FFE738CC740797779E89800700A4D4141D8AB75EB4DCA135978A3",
));

/// Edwards `2*d` value, equal to `2*(-121665/121666) mod p`.
pub(crate) const EDWARDS_D2: FieldElementR0 = FieldElementR0(U256::from_be_hex(
    "2406D9DC56DFFCE7198E80F2EEF3D13000E0149A8283B156EBD69B9426B2F159",
));

/// One minus edwards `d` value squared, equal to `(1 - (-121665/121666) mod p) pow 2`
pub(crate) const ONE_MINUS_EDWARDS_D_SQUARED: FieldElementR0 = FieldElementR0(U256::from_be_hex(
    "029072A8B2B3E0D79994ABDDBE70DFE42C81A138CD5E350FE27C09C1945FC176",
));

/// Edwards `d` value minus one squared, equal to `(((-121665/121666) mod p) - 1) pow 2`
pub(crate) const EDWARDS_D_MINUS_ONE_SQUARED: FieldElementR0 = FieldElementR0(U256::from_be_hex(
    "5968B37AF66C22414CDCD32F529B4EEBD29E4A2CB01E199931AD5AAA44ED4D20",
));

/// `= sqrt(a*d - 1)`, where `a = -1 (mod p)`, `d` are the Edwards curve parameters.
pub(crate) const SQRT_AD_MINUS_ONE: FieldElementR0 = FieldElementR0(U256::from_be_hex(
    "376931BF2B8348AC0F3CFCC931F5D1FDAF9D8E0C1B7854BD7E97F6A0497B2E1B",
));

/// `= 1/sqrt(a-d)`, where `a = -1 (mod p)`, `d` are the Edwards curve parameters.
pub(crate) const INVSQRT_A_MINUS_D: FieldElementR0 = FieldElementR0(U256::from_be_hex(
    "786C8905CFAFFCA216C27B91FE01D8409D2F16175A4172BE99C8FDAA805D40EA",
));

/// Precomputed value of one of the square roots of -1 (mod p)
pub(crate) const SQRT_M1: FieldElementR0 = FieldElementR0(U256::from_be_hex(
    "2B8324804FC1DF0B2B4D00993DFBD7A72F431806AD2FE478C4EE1B274A0EA0B0",
));

/// `APLUS2_OVER_FOUR` is (A+2)/4. (This is used internally within the Montgomery ladder.)
pub(crate) const APLUS2_OVER_FOUR: FieldElementR0 = FieldElementR0(U256::from_be_hex(
    "000000000000000000000000000000000000000000000000000000000001DB42",
));

/// `MONTGOMERY_A` is equal to 486662, which is a constant of the curve equation
/// for Curve25519 in its Montgomery form. (This is used internally within the
/// Elligator map.)
pub(crate) const MONTGOMERY_A: FieldElementR0 = FieldElementR0(U256::from_be_hex(
    "0000000000000000000000000000000000000000000000000000000000076D06",
));

/// `MONTGOMERY_A_NEG` is equal to -486662. (This is used internally within the
/// Elligator map.)
pub(crate) const MONTGOMERY_A_NEG: FieldElementR0 = FieldElementR0(U256::from_be_hex(
    "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF892E7",
));

/// `L` is the order of base point, i.e. 2^252 +
/// 27742317777372353535851937790883648493
pub(crate) const L: ScalarR0 = ScalarR0(U256::from_be_hex(
    "1000000000000000000000000000000014DEF9DEA2F79CD65812631A5CF5D3ED",
));

/// `R` = R % L where R = 2^261
pub(crate) const R: ScalarR0 = ScalarR0(U256::from_be_hex(
    "0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFD656EB3C98B3BDF026334C2E60714DF9ED",
));

/// The Ed25519 basepoint, as an `EdwardsPoint`.
///
/// This is called `_POINT` to distinguish it from
/// `ED25519_BASEPOINT_TABLE`, which should be used for scalar
/// multiplication (it's much faster).
pub const ED25519_BASEPOINT_POINT: EdwardsPoint = EdwardsPoint {
    X: FieldElementR0(U256::from_be_hex(
        "216936D3CD6E53FEC0A4E231FDD6DC5C692CC7609525A7B2C9562D608F25D51A",
    )),
    Y: FieldElementR0(U256::from_be_hex(
        "6666666666666666666666666666666666666666666666666666666666666658",
    )),
    Z: FieldElementR0::ONE,
    T: FieldElementR0(U256::from_be_hex(
        "67875F0FD78B766566EA4E8E64ABE37D20F09F80775152F56DDE8AB3A5B7DDA3",
    )),
};

/// The 8-torsion subgroup \\(\mathcal E \[8\]\\).
///
/// In the case of Curve25519, it is cyclic; the \\(i\\)-th element of
/// the array is \\(\[i\]P\\), where \\(P\\) is a point of order \\(8\\)
/// generating \\(\mathcal E\[8\]\\).
///
/// Thus \\(\mathcal E\[4\]\\) is the points indexed by `0,2,4,6`, and
/// \\(\mathcal E\[2\]\\) is the points indexed by `0,4`.
pub const EIGHT_TORSION: [EdwardsPoint; 8] = EIGHT_TORSION_INNER_DOC_HIDDEN;

/// Inner item used to hide limb constants from cargo doc output.
#[doc(hidden)]
pub const EIGHT_TORSION_INNER_DOC_HIDDEN: [EdwardsPoint; 8] = [
    EdwardsPoint {
        X: FieldElementR0::ZERO,
        Y: FieldElementR0::ONE,
        Z: FieldElementR0::ONE,
        T: FieldElementR0::ZERO,
    },
    EdwardsPoint {
        X: FieldElementR0(U256::from_be_hex(
            "1FD5B9A006394A28E933993238DE4ABB5C193C7013E5E238DEA14646C545D14A",
        )),
        Y: FieldElementR0(U256::from_be_hex(
            "7A03AC9277FDC74EC6CC392CFA53202A0F67100D760B3CBA4FD84D3D706A17C7",
        )),
        Z: FieldElementR0::ONE,
        T: FieldElementR0(U256::from_be_hex(
            "6CE244C360A26BEB303276D192F8AF0ABA993D74CDFA85962D0D253EDE7E8781",
        )),
    },
    EdwardsPoint {
        X: FieldElementR0(U256::from_be_hex(
            "547CDB7FB03E20F4D4B2FF66C2042858D0BCE7F952D01B873B11E4D8B5F15F3D",
        )),
        Y: FieldElementR0::ZERO,
        Z: FieldElementR0::ONE,
        T: FieldElementR0::ZERO,
    },
    EdwardsPoint {
        X: FieldElementR0(U256::from_be_hex(
            "1FD5B9A006394A28E933993238DE4ABB5C193C7013E5E238DEA14646C545D14A",
        )),
        Y: FieldElementR0(U256::from_be_hex(
            "05FC536D880238B13933C6D305ACDFD5F098EFF289F4C345B027B2C28F95E826",
        )),
        Z: FieldElementR0::ONE,
        T: FieldElementR0(U256::from_be_hex(
            "131DBB3C9F5D9414CFCD892E6D0750F54566C28B32057A69D2F2DAC12181786C",
        )),
    },
    EdwardsPoint {
        X: FieldElementR0::ZERO,
        Y: FieldElementR0(U256::from_be_hex(
            "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEC",
        )),
        Z: FieldElementR0::ONE,
        T: FieldElementR0::ZERO,
    },
    EdwardsPoint {
        X: FieldElementR0(U256::from_be_hex(
            "602A465FF9C6B5D716CC66CDC721B544A3E6C38FEC1A1DC7215EB9B93ABA2EA3",
        )),
        Y: FieldElementR0(U256::from_be_hex(
            "05FC536D880238B13933C6D305ACDFD5F098EFF289F4C345B027B2C28F95E826",
        )),
        Z: FieldElementR0::ONE,
        T: FieldElementR0(U256::from_be_hex(
            "6CE244C360A26BEB303276D192F8AF0ABA993D74CDFA85962D0D253EDE7E8781",
        )),
    },
    EdwardsPoint {
        X: FieldElementR0(U256::from_be_hex(
            "2B8324804FC1DF0B2B4D00993DFBD7A72F431806AD2FE478C4EE1B274A0EA0B0",
        )),
        Y: FieldElementR0::ZERO,
        Z: FieldElementR0::ONE,
        T: FieldElementR0::ZERO,
    },
    EdwardsPoint {
        X: FieldElementR0(U256::from_be_hex(
            "602A465FF9C6B5D716CC66CDC721B544A3E6C38FEC1A1DC7215EB9B93ABA2EA3",
        )),
        Y: FieldElementR0(U256::from_be_hex(
            "7A03AC9277FDC74EC6CC392CFA53202A0F67100D760B3CBA4FD84D3D706A17C7",
        )),
        Z: FieldElementR0::ONE,
        T: FieldElementR0(U256::from_be_hex(
            "131DBB3C9F5D9414CFCD892E6D0750F54566C28B32057A69D2F2DAC12181786C",
        )),
    },
];
