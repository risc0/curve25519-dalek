use super::field::FieldElementR0;
use super::scalar::ScalarR0;
use crate::edwards::EdwardsPoint;

use crypto_bigint::U256;

#[cfg(feature = "precomputed-tables")]
use crate::{
    backend::serial::curve_models::AffineNielsPoint,
    edwards::EdwardsBasepointTable,
    window::{LookupTable, NafLookupTable8},
};

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

/// Table containing precomputed multiples of the Ed25519 basepoint \\(B = (x, 4/5)\\).
#[cfg(feature = "precomputed-tables")]
pub static ED25519_BASEPOINT_TABLE: &'static EdwardsBasepointTable =
    &ED25519_BASEPOINT_TABLE_INNER_DOC_HIDDEN;

/// Inner constant, used to avoid filling the docs with precomputed points.
#[doc(hidden)]
#[cfg(feature = "precomputed-tables")]
static ED25519_BASEPOINT_TABLE_INNER_DOC_HIDDEN: EdwardsBasepointTable = EdwardsBasepointTable([
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "07cf9d3a33d4ba65270b4898643d42c2cf932dc6fb8c0e192fbc93c6f58c3b85",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "44fd2f9298f81267a5c18434688f8a09fd399f05d140beb39d103905d740913e",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "6f117b689f0c65a85a1b7dcbdd43598c26d9e823ccaac49eabc91205877aaa68",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "590c063fa87d2e2e5aa69a65e1d607029f469d967a0ff5b59224e7fc933c71d7",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "6bb595a669c92555e09e236bb16e37aa8f2b810c4e60acf68a99a56042b4d5a8",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "701af5b13ea50b73500fa0840b3d6a3136c16bdd5d9acf7843faa8b3a59b7a5f",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "7a164e1b9a80f8f4c11b50029f016732025a8430e8864b8aaf25b0a84cee9730",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2ab91587555bda628131f31a214bd6bd3bd353fde5c1ba7d56611fe8a4fcd265",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "5a2826af12b9b4c6d170e5458cf2db4c589423221c35da6214ae933f0dd0d889",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "680e910321e58727ca348d3dfb0a92656765c6f47dfd2538287351b98efc099f",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "27933f4c7445a49ac3e8e3cd06a05073327e89715660faa995fe050a056818bf",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "7f9d0cbf63553e2b5ddbdcf9102b44946e9e39457b5cc1725a13fbe9c476ff09",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "2945ccf146e206ebdd1beb0c5abfec448d5048c3c75eed02a212bc4408a5bb33",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "154a7e73eb1b55f3e33cf11cb864a087d50014d14b2729b77f9182c3a447d6ba",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "43aabe696b3bb69ab41b670b1bbda72d270e0807d0bdd1fcbcbbdbf1812a8285",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "51e57bb6a2cc38bd8065b668da59a7369b27158900c8af883a0ceeeb77157131",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "38b64c41ae417884bb085ce7204553b9575be28427d22739499806b67b7d8ca4",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "10b8e91a9f0d61e353e4a24b083bc144be70e00341a1bb0185ac326702ea4b71",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "461bea69283c927e71b2528228542e497470353ab39dc0d26b1a5cd0944ea3bf",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "1d6edd5d2e5317e09dea764f92192c3a6ca021533bba23a7ba6f2c9aaa3221b1",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "7a9fbb1c6a0f90a7529c41ba5877adf3b3035f47053ea49af1836dc801b8b3a2",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "0915e76061bce52fb1339c665ed9c3236cb30377e288702c59b7596604dd3e8f",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "3a9024a1320e01c32c2741ac6e3c23fb963d7680e1b558f9e2a75dedf39234d9",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "26907c5c2ecc4e95636412190eb62a32b8a371788bcca7d7e7c1f5d9c9a2911a",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "6dd37d49a00eef3952dfb76ba8637a5851d0b696768931152eccdd0e632f9c1d",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "30d76d6f03d315b9850c1fe95b42d1c4a865c49f0bc6823aed5b635449aa515e",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "10c697112e864bb0b4739ea4694d3f26fb53d680928d7f696c4444172106e4c7",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "03bf9baf550806ef7464d3a63b11eddc6a3d4ae37a2042470ca62aa08358c805",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "25e61cabed66fe090e00dfc846304590265d4fad19ad7ea26493c4277dbe5fde",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "566d78634586e22ca0b63dedcc1268f56f5873ecb459747e3f13e128cc586604",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "14fba5f34793b22a680ae240731aee586c64112af31667c3a1054285c65a2fd0",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "33975bca5ecc35d91cb5ec0f7f7fd2dbbc8e56d5a89bc4511637a49f9cc10834",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "14829cea83fc526c2fc3f2b67b61131e593e5e84c9c800573cd746166985f7d4",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "05fc3bc4535d7b7ebf6556cece1d4f80e656ddb940a477e321e70b2f4e71ecb8",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2fd9c71e5f7581739e0c5d613c85e88b6c744e30aa4eb5a7ff437b8497dd95c2",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "373767475c651f0433a4bc83a9be81953495638ced3b30cf24b8b3ae52afdedd",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "6bf5905730907c8c9e38140c8910bc60ef12144016c15535634095cb14246590",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "1fbea56c3b18f9994363f05215f03baeb307166f96f4d0272fba99fd40d1add9",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "02521cf67a635a566f52d7b89aa29a5006409ff7bac3a77e0fa778f1e1415b8a",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "001753d9f7cd6cc44af8224d00ac824ae8f894b196079aceb1146720772f5ee4",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "3fa00a7e71382cedd4618688bfe107ce8f98e75c0fdf5a66513fee0b0a9d5294",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "12b5fe2fa048edb6aad7d1f9a091f2851dde87dab49738583c69232d963ddb34",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "6f7e93c20796c7b88c409dc0751c8bc34b66d323504b8913df2b7c26ad6f1e92",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "749b76f96fb1206fd2047261ff28c56173b9826badf35bed71f0fbc496fce34d",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "0fcec10f01e0215161a808b5eeff6b66c12351f1bee49c991f5af604aea6ae05",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "413779166feab90a6cc8067e820c214d2b020e7493d8de0a3df2d29dc4244e45",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "4901aa7183c511f302441c5a887fd0d221fcaea231ad777e644d58a649fe1e44",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "41bb887b726d1213f760b0f91e06d939ce0f7a7c246299b408b1b7548c1af8f0",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "0a871e070c6a9e1ddae2c90c354afae764889d3d0a85b4c87e234c597c6691ae",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "4eb728c12fcdbdf77c3a8a18a13b603e1d48dad415b52b2540e87d44744346be",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "3ee7300f2685d4640d61ade219d59e3c736bae3a5bdd42603301b5994bbc8989",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "7d47c6a2cfb89030a1065e1de3052b74e5c6fa59639c46d743fa7947841e7518",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "13815762979125c23c99975d92e187ca8016115c610b1eacf5d255e49e7dd6b7",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "148cf58d34c9ec8071ec621026bb81579d3e749a91546f3c3fdad0148ef0d6e0",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "278febad4eaea1b99fd10b6d6960a88d56c345bb88f3487fe2572f7d9ae4756d",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "3ff2fa1ebd5dbbd45ca1bc2a89611854469984bef6840aa946a492f67934f027",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4104dd02fe9c677b39115291219d3c528c21949c20290c98b1aa681f8c933966",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "0165b5a48efca4816524c12a409e2af521a8b6c90ce44f3581214e06db096ab8",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2c863b00afaf53d594cb6101fa52b666a1fa0c3398a33ab572b2bf5e1124422a",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "591b67d9bffec8b8695e290658aa2b8f12eff984cd2f7cc0f190a474a0846a76",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "489b4f867030128b61081136c29f05ede465e5faa18c641e99b9b3719f18b55d",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "3de02ec7ca8f7bcb727033c09ef01c885979515eabf3ec8a312f0d1c80b49bfa",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "49f5fbba496cbebf3d7eabe7190baa24e16253b46116a861d232102d3aeb92ef",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "06a1a6c28867515b91a352f6515763eb8a4d86acc5884741155d628c1e9c572e",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "5604a86dcbfa6e7492c294c1307c0d1cdc40dd70bc6473eb30949a108a5bcfd4",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "20989e89fe2742c69f031a6018acf6d172541140e0418b517288d1d47c1764b6",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "730b9950f70595d3640a4c1661cbf45a5621dc077acb2bdf1674278b85eaec2e",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0403ed1d0ca67e29a279d864d207e3a032857c2ca54fd892499777fd3a2dcc7f",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "5bd7454308303dccf7cb46fa16c035cec5e6c8cf98246f8dc94b2d35874ec552",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "5bb7db123067f82c9d1e3da8ada3d762c64c89a2bdcdddc985c4932115e7792a",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "133a78007380ad8366b8b66e4fc072367f6b54656335c1817f9ad19528b24cc2",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "740dca6d58f0e0d2182360779bd5477004ec21d6211952ee0961f467c6ca62be",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "2c03f5256c2b03d9dbaa99eae3d9079fb7b5270ec281439d231a8c570478433c",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "161d25fa963ea38d05710b2ab95459c4c3fffaf306ec08b7df48ee0752cfce4e",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "699468bdbd96bbaf31903d77257ef7f9307b0130cf0c5879790f18757b53a47d",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "293e1c4e6c4a2e3a9b48246634fdea2f485064c22fc0d2ccd8dd3de66aa91948",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "525219a473905785d31ffdda4a47b37f7cef0114a47fd6f7bd1f2f46f4dafecf",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "5b605c447f032823b04589af461c3111703778b5dca15da0376e134b925112e1",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "2f12fef4cc5abdd55542ef161e1de61a866a579e75e349623be9fec6f0e7f04c",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "24a76dcea8aeb3ee0001256502e2ef77e7f0100c923b8fccb965805920c47c89",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "5e607b2518a43790c6cf144178cff66810d06e7f40c9a4070a4522b2dfc0c740",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "201f33139e457068d24526802e0f26dbe3c42d40aed3e400a02c431ca596cf14",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "5067acab6ccdd5f7e1b3ff4f66e61d6e35cfa74fc36258a258b31d8f6cdf1818",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "397cba8862460375d5220eb02e25a4a818b14964017c0006fd527f6b08039d51",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "771b4022c1e1c252ffa9c0f885a8fdd5a6619420dde12af17815c3fbc81379e7",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2339d320766e6c3a222fd491721d5e26e23aa18de9a9797630c13093f05959b2",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "331a189219971a76d06bc31b1ea283b3f5ac9b71f9d4cf08d87dd986513a2fa7",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "1ac9619ff649a67b84edc1c11180f7c45bcbe28868074a9e26512f3a9d7572af",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "31167c6b83bdfe21e3d4e81b9041d91c9c36c7de61c775cff5166f45fb4f80c6",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "612436341f08b111fc9d71844a6250c85068343bee9ce987f22b3842524b1068",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "1522aa3178d904457f8bf1b8a3a06ba49ddfb7009bd3fd358b6349e31a2d2638",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "46ebe2309e5eff40793d2c67d00f9bbc09fea5f16c07dc20d99d41db874e898d",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "45fe70f50524306ce8c83391b646f227dafe409ab72d6d102c382f5369614938",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "16619f6db57a22456fbb45d2f5c9d4b805f007c83f630ca262f24920c8951491",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "428d1623a0e392d407fb51cf3d0b8fd45b68d076ef0e2f20da4875a6960c0b8c",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "1d81592d60bd38c6deb8de4643d1bc7da82219c376a5caac084f4a4401a308fd",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "3a5f6d51b0af8e7c3a70e77c1d330f9b6ff0678bd168bab28765b69f7b85c5e8",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "20234a7789ecdcf07f193f2d4cce0f7d17e02f6aebabdc5761368756a60dac5f",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "7cd682353cffe366f62a4a20b3e41170071c34f9d51ed16076d20db67178b252",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "3ef0253b2b2cd8ff5759389d336025d942d92d183cd7e3d3a665cd6068acf4f3",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2838c8863bdc0943d9921012e96e60002a846a32ba403b6e0be1a45bd887fab6",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "21dcb8a606a8281282cfae8af4ab419dfa496b4115c577abd16bb0cf4a465030",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "5d840dbf6c6f678bb2cc023743f3d97f8203607e629e18899a8d00fabe7731ba",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0bc3d08194a31dab125b4d4c12ee2f9c2540096ed42aa3cb5c6004468c9d9fc8",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "3a4276232ac196dd57bbba997dae20ab6eb02da6b9e165c7706e380d309fe18b",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "0770c9e824e1a9f980acffc075aa15fe5fcfc41fc6282dbd3bf8c172db447ecb",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "1ff177cea16debd1be9f00219c58e45d898a19e3dfb9e5454b42432c8a7084fa",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "39f264fd41500b1ee7300919303e3e89860984e91b3a7924cf61d99a45b5b5fd",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "65c621272c35f14ec3c908942ca6f1ffa46dfce1dfe01929d19b4aabfe097be1",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "1712d73468889840a0e91b8e93597ba9bd94376a2b9c139ca7ad3417dbe7e29c",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "22f9800ab19ce2720419a93d2e1cfe834d103356a125c0bbe72b89f8ce3193dd",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "4171a4d38598cab4640f64b987bdf37bb912cebe34a5494142029fdd9a6efdac",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "13f416cd647626e5553d48b05f24248fe3e9c022a5504715605a368a3e9ef8cb",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "508214fa574bd1abfbd291ddadda539223006f6fb000b807fa2758aa99c94c8c",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "3a7758a4c86cb44727c576756c683a5ab2102888bcf3c965461a15bb53d003d6",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "22f960ec6faba74bcbde26462c14af94a65a6739511d77c4c20269153ed6fe4b",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "5d9fd15f8de7f49412248c90f3115e651dae21df1dfd54a6548111f693ae5076",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "3bc187fa47eb98d8e164ba772e9c16d48e3a9028432e96153f244d2aeed7521e",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "332f35914f8fbed3a9e18fc5ccaee24b6a379aefd7c7b533031408d36d63727f",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "450d81ce906fba03d77832b53a660188998ab7cb6c46d1256d470115ea86c20c",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "4cbad279663ab10897bdc55be7f4ed29f86d18f5ee1c63edd074d8961cae743f",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2de25d4ba6345be16f56d155e88f5cb2aa4f21d7413c8e836e7bb6a1a6205275",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "7d7fbcefe2007233b1a3974b5f3a6419c525c20afb288af880d19024a0d71fcd",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "4f9cd196dcd8d4d780c61d36421c3058c781a29a2a9105abcd7c5dc5f3c29094",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "5975435e87b75a8df68a2fbc1b03762c866c68c4d5739f16faef1e6a266b2801",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "7ccdd084387a0307ba029cad5c1c0c17d0d058241ad17a63199297d86a7b3768",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "3593ca848190ca44a88dec86620bda18cdae007a1ab32a999b0c84186760cc93",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "428bd0ed61d0cf53a9c0c1b4fb68c677ae153d50948240bddca6422c6d260417",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "27398308da2d63e68c52545b53fdbbd1d4d8c33565d8facd9213189a5e849aa7",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "572c2945492c33fdffb9d9b5cd27daf70fa25866d57d1bdeb9a10e4c0a702453",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "269597aebe8c3355bb07ab1a79da03efbd50f3603278ccc942c38d28435ed413",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "7f985498c05bca80a22c8830aa5dda0ce4dfe8d3e3baaefbc77fc745d6cd30be",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "30f2653cd69b12e7eec24fbc873fa0c208045a45cf4dfba6d35615520fbf6363",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "76c2ec470a31f3043da3c39f23fc921c8005ad1b7b54a2883849ce889f0be117",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "2f1273f1596473daa920c01e0e6fac7046179b60db276bcb8a08c938aac10c85",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "38ac1997edc5f784ead1a69ebc59616206d6b5a4f1d442e730488bd755a70bc0",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "5d9e572ad85b69f241d98a8287728f2efd5274904a6aab9f4739fc7c8ae01e11",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "22dfcd9cbfe9e69cacacc011454dde49747d06867e9b858c0666b517a751b13b",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "0fe9877824cde472797cb29413f5cd322ee3baecd259f96956ec59b4103be0a1",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "6b2916c05448c1c78f6b258c322a961fad8e665facbb43338ddbd2e0c30d0cd9",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4c303f307ff00a1766083dff6578f8154ea3cd822e6dac0e7edb34d10aba913b",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "416b151ab706a1d5130a155fc2e2a7f8ecd27aa46fbbec9329fc03580dd94500",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0d61b8f78b2ab7c4c6c6e78c1e6a5cbfc5d377b739773bead30a3bd617b28c85",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "191a2af74277e8d2afe62fda1b57e0abbd07e5cd58e44b2056a8d7efe9c136b0",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "237e7dbe00545b93f052210eb7a0da24ded303d4a63607d69fe62b434f460efb",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "7e2e0e450b5cc908b8b9c36fb5b23ee72b9725ce2072eddece16f74bc53c1431",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "71afa699b11155e375320f1583e47f22231094e69f0bfd10013575ed6701b430",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "00731fbc78f89a1c9b84bf5fb2c9be9551e87a1f3b38ef10ea423c1c473b50d6",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "021142e9c2b1c28e0f435ffda9f759fec65839eaafa141e665ce6f9b3953b61d",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4c4d6f3347e15808b6dae0836bba15e3bf960c225ecec119e430c71848f81880",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "38bf9500a88f9fa86ec7b6c4779176be6b916227b0b9f51b2f0cddfc988f1970",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "193fddaaa7e47a22dbde712bf7ee0cdf6c75f5a651403c1418f7eccfc17d1fc9",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "6a6fb99ebf0d49695080f58239241276a2f61e5a18d1462c1fd2c93c37e8876f",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "136fda9f42c5eb1090a92a831dcf5d8c939d7010f286ff8eeeb122b5b6e423c6",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "75f76914a31896ead71d11378f71acc12416bb38f893f09d6a46c1bb560855eb",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "022183510be8dcba2a87d8a5c3bb588a0f364b9d9ff82c08f94cdfb1a305bdd1",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "1387c441fd40426c22bbfe52be927ad3b063de9ec47da45f9d5a710143307a7f",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "6a071ce17b806c030d13a6e610211e3da08ed880ca7c58304af766385ead2d14",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "3171b26aaf1edc920d7b4848bb477ca0722b5a3d7f0e4413b5d3c3d187978af8",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "6c514a63344243e9d4a1f89353ddbd58a6bf14d61770a4f1a60db7d8b28a47d1",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "221fd4873cf0835a4f55fe37a4875150ff7bb84c2275e119a92f319097564ca8",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "48daa596fb924aaafce0dd4c410f030efb73e0e9ba0a032d2322204f3a156341",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "58c837fa0e8a79a9cdf5b88f346277ac9941f9e3ef41820614f61d5dc84c9793",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "676dd6fccad84af731afc708d21e17cea847254b2e38aca06eca8e665ca59cc7",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "1c4f73f2c6a57f0adcc2e77d4935d66a1ddcbbf37b56a01b0cf9688596fc9058",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "70459adb7daf675ceb1d79c9781cc7e573dfc9b4c3c1cf61b36e706efc7c3484",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "795ac80d1bf64c42f421c3832fe33848829d4ce054c663ad0e7a4fbd305fa0bb",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "5fe162848ce21fd39fdf9ee51f8c78dc572696234b02dcca1b91db4991b42bb3",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "2b44c043677daa35dc0bffab21687891b8dedd70687df2e72879852d5d7cb208",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "4789d446fc9172329300cfd23b50f22d49be7dc70d71cd4f4e59214fe194961a",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "60c52eef2bb9a4e43eacbbcd484f9067fac6d18e99daf4671a1c87ab074eb78e",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "70d77248d9b6676defbc4056ba492eb244c7699b54a48cab702bc5c27cae6d11",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "10bc9c312ccfcaab0e4c16b0d53028f5b06b9237c9f3551a0b5d89bc3bfd8bf1",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "63755bd3a976f413794513e4708e85d198699ef4ed1781e0aa8ae84b3ec2a05b",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "2763751737c52a56508e5b9c0fa1020f5dda7d5ec165bbd83dc7101897f1acb7",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0d8cc1c48bc16f849ff9f1fdbe69b890356f75909ee63569b55fa03e2ad10853",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "70c2dd8a7ad166e7cfa86230d43c4956f0b44e7e77b460a5029402d36eb419a9",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "32b8613816a53ce5e40982e00d85256474252f0ad776817a91d4967db8ed7e13",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2e0fac63639484959733c1f367e09b5cee2e7ea946c6518d656194509f6fec0e",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4179215c735a4f41f89fd4d9a0e4db2e6ac83a67087886d079e7f7bee448cd64",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "31fa85662241c286278b141fb3d38e1fb7ef7eb6559dd6dce4ae33b9286bcd34",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "700344a30cd99d76e13be033a906d90297fb8ac347d39c708c7094e7d7dced2a",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "24bb2312a99524899bc1b7e12b389123c12029879833502daf826c422e3622f4",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "0d1d2af9ffeb5d168945df99a3ba1bad687284c304fa679441f80c2af5f85c6b",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0fee3e871e1880088ebd434376cfbcd23cb49418461b4948b1a8ed1732de67c3",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "16acd79718531d764004197ba79ac19330b822a159226579a9da8aa132621edf",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "164ed34b1816170016e24e62a342f50494e19ead5f90febac959c6c57887b6ad",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "123e0ef6b93023093ecea07916b3963763462a36a432245a72df72af2d9b1d3d",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "78da0fc61073f3eb877bf6d3b9a4de2761ae2cea3a911513487ed94c192fe69a",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "054aa4b316b38ddd1100f1584801797371f77e151ae9e7e6a29f80f1680c3a94",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2d25deeb256b173a2419afbc06c28bdd2c47e31870f01a8e5bf15d28e52bc66a",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "134610a6ab7da7602aeb1d2a666eec170b28789c66e54dafdfc8468d19267cb8",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "654878cba97cc9fbb1b73bbcdabc06e5548991878faa60f1cd2a65e777d1f515",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2102fdba2b20d65009207a1d717af1b95397da89e575f51b51138ec78df6b0fe",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "0ad725db29ecb2ba3a1af517aa7da41536bca7681251ad29969ee405055ce6a1",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "4f675f5302399fd977afc6624312aefa537d5268e7f5ffd7fec7bc0c9b056f85",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "4aefcffb71a036501af07a0bf7d15ed7b67544b570ce1bc5dc4267b1834e2457",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "0bccbb72a2a86561870a6eadd0945110cd2bef118998483bc32d36360415171e",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "6678fd69108f37c23bc7f6c5507031b0e0397b82fee89f7e186d5e4c50fe1296",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "4024f0ab59d6b73eb092e031bb5b6df286e7e63565147dcd185e962feab1a9c8",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "2d42e2108ead47014f73cc9f789eaefc07f68c48572d33f21586fa31636863c2",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "420bf3a79b423c6e1bb687ae752ae09f914e690b131e064c21717b0d0f537593",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "676b2608b8d2d322eba13f07084550106155985d313f4c6a97f5131594dfd29b",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "745d2ffa9c0cf1e07bff0cb1bc3135b08671b6ec311b1b808138ba651c5b2b47",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "06e15be54c1dc839d3c209c3c8756afab1db8827997bb3d06036df5721d34e6a",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "1ae23ceb960cf5d0d511c70edf0155dbea5b260826479d81bf525a1e2bc9c8bd",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "58ded861278ec1f77dc41549dab7ca0532351cb5ceb1dab05b725d871932994a",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "08ba696b531d5bd833809107f12d157348eeef8ef52c598c2dfb5ba8b6c2c9a8",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2ff39de85485f6f95ce382f8bc26c3a8c8c976c5cc454e49d8173793f266c55c",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "120633b4947cfe54ea3d7a3ff1a671cb04e05517d4ff481177ed3eeec3efc57a",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "07433be3cb393bdee11e761911ea79c6de237b6d7e6fbe0682bd31474912100a",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "35d30a99b4d5918570be739594f0a4c04ee7b13cecebfae80b94987891610042",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "2d873ede7af6da9f583381fd5a76847c575d3de4b05c51a3ff7944c05ce997f4",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "0543618a0160025318a275d3bae21d6ca20d59175015e1f5aa6202e14e5df981",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2e773654707fa7b6b093fee6f5a64806fab8b7eef4aa81d9157a316443373409",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4b1443362d07960d04202cb8a29aba2caa6f0a259dce46930deabdf4974c23c1",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "17038418eaf66f5c2747aff47812196530f6269264c635fb967c54e91c529ccb",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "582f446752da63f73ef06dfc713eaf1c44157e25f50c2f7eccc4b7c7b66e1f7a",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "0c2a1c4bcda28dc9b21ef18b4e5a1364a81042e8a4488bc4c6317bd320324ce4",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "5a6a9b30a314dc83c63bd212d55cc5ab0d6d907dbe1c8d22edc4814869bd6945",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "7c558bd1c6f64877d15b0272fbb2d28fb2269e3edb87c059d24dc7d06f1f0447",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "0404a5ca0afbafc3a50c3a791cbc5fa412bb628ac35a24f0d0ec1524d396463d",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "39527516e7f8ee9804343fd83d5d6967b5c6f728e350598b62bc9e1b2a416fd1",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "46395bfdcadd9633574b046b668fd2deccbad0cb5b265ee88c1f40070aa743d6",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "76579a29e822d016efd4bef154d56fea9c7745bcd1005c2a117fdb2d1a5d9a9c",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "02c514bb2a2777c1b5512887750d35ced832284993de80e1333cb51352b434f2",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "43a384dc9e05bdb13ed65f11ec224c1b23cd51a2bca9a37f45b68e7e49c02a17",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "1160920961548059313916d7a9b0d253fb8bd37ef6b54b53684bd5da8bf1b645",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "2f637d7491e6e0427dc21bf9d4f1802175c02ca7655c35637a385616369b4dcd",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "351e125bc5698e0be9ef63ca453d5559da529f4c8413598fb44d166929dacfaa",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "7f182d06e7ce2a9a71dee19ff9a699fbd603037ac8ab8961d4b49b461af67bbe",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "44acc043241c5217d358254d7f46903caa58e8f4d484b8d809454b728e217522",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "67d4ac8c343e93b0095519d347cd0edacb5a4a5515edc5437a7c8e64ab0168ec",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "556d1c8312ad71bd4adca1c6c96b46848b35fed4918313e11c7d6bbb4f7a5777",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "097abe38cc8c7f05f8b2d0556a99465d0faff82310a3f3dd81f06756b11be821",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "746c6c6d1d824eff4f21f3cb0773646e31f7073e15a3fa3417ef40e30c8d3982",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "577e14a34bee84bd022c3809f7ccebd24c4369559bdc1d430c49c9877ea52da4",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "705304b8fb009295124a5977c0c8d1fff46a4fda060f221194fecebebd4dd72b",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "60fa7ee96fd78f42c1e13e826b6d00e9f2fafa103791a5f5f0e268ac61a73b0a",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "21398e0ca16353fd670b958cb4bd42ecf3c3053e5fad31d8b63d1d354d296ec6",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "1bbfb284e98f7d4ed2ceaa6161b7a0235eac72135c8dad722798aaf9b4b75601",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "5aa3ed9d95a232e68f93b503a53db36e5ae2ba0bad48c0b489f5058a382b33f3",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4a07e14e5e8957cc65053299d9506eeecb2b125472c78036656777e9c7d96561",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "50af7aed84afa08119928d32a7c86aadfd38dade6447f017240b58cdc477a49b",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "3f0bac391d313402bba5edde925c77fda315d76f3c6ec7714ee412cb980df999",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "1921a316baebd006780205810badd6d929982621216109b26e4fde0115f65be5",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "7b049deca062c7f53e9a0bac255c0ed9566a0eef60b1c19cd75aad9ad9f3c18b",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "6458df41e273aeb00738f1d436c24df72c296beb4f76b3bd89422f7edfb870fc",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "6093dccbc2950e64786004c312c5dd87758879330fedbe93dccbe37a35444483",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "6e3180c98b647d90973376abb62d06953199c2b6780fb8546bdeeebe6084034b",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "57d1ea084827a97c43b9f2e1718f453b36d0a5d8b3e739331ff39a8585e0706d",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "363e999ddd97bd18f7b4de82b2216130a4c1596d93a88baaee7ab6e7a128b071",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "231f979bc6f9b35590cb3c6e3cefe931769b7255babcaf602f1848dce24baec6",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "48ee9b78693a052bb42f6801b58cd330976eb35508e4c8cf96a843c135ee1fc4",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "079bfa9b08792413b78d7009c14fb466b04bb030fe208d1f5c31de4bcc2af3c6",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "1ef4fb159470636bd76dac63d10854750aa08b7877f63952f3c9ed80a2d54245",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "038c77f684817194ed3cf12d0b356480843964233da95ab0e3903a51da300df4",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "575ee92a4a0bff5672b2df349810219959590a4296d0cdc2854e5ee65b167bec",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "200a1e7e382f581b389e3b262b8906c2c3af1227a533b9d85d46bc450aa4d801",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "43d4e7112cd3fd0005babd5752f733de30e170c299489dbdd4c080908a182fcf",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "050eca52651e4e38fe2b85d9567197f571bc989b056652c0518db967eaf93ac5",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "6151c09fa131ae574cb179b534eca79f9b19bbfe153ab49797ac397660e668ea",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "4b0ee6c21c58f4c6fcd97ac9ed847b3de9f5045eff703b9bc3431ade453f0c9c",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "1fef24fa800f030b11b2bb8712171709dd262ee02ab4ee7a3af55c0dfdf05d96",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "28005fe6c8340c17dca1896c4de5bae58d90b806c2d2460422d2aff530976b86",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "544d49292fc8613ead200b09fb3a17b20f9495303fd7641837d653fb1aa73196",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "72e472930f316dfaf75bbbcd66d94b365c1bff9425107da16aefba9f34528688",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "1fee7f000fe064383c85e79728d044507aaa4d865f6566f007f3f635d32a7627",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "5a9d2e8c2733a34cfd9daea603efde02b1502a0b23450ee12695208c9781084f",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "00f94051ee0405437b4ad5cdd24a88eca4daf2491434cdbd765305da03dbf7e5",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "47b7ffd25dd40452ce6998bf6e0b1ec5583ed0cf3db766a7d7ef93bb07af9753",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "07d79c7e8beab10db36c316c6e42b83cf21c8b9bb0471b068d356b23c3d330b2",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "1c3520a35ea64bb60d57242bdb1fc1bf8a066b3ae1eec29b87fbfb9cbc08dd12",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "38750c3b66d12e55b4956a9e17c709901fbb231d12bcd87ecda86f40216bc059",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "1c3d05775d0ee66f90c3b6019882e3963e61c3a13838219b80d253a6bccba34a",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "0621e2c7d330487c21014fe7744ce029cbc0c73c2b5df671692ef1409422e51a",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "5d8e589ca1002e9d25923071e9aaa3b454dfafb9e17ce196b7ae1796b0dbf0f3",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0007d6097bd3a5bc6526483765581e3090ea48c1c69f9adcaf9860cc8259838d",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "1583d7783c1cbf860a961438bb51e2efb2d3c363588f2e3ec0bf1d950842a94b",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "4958064e83c5580a16e12b5fbe5b87261d1b679ef72cc58f90034704cc9d28c7",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "252a5f2e81ed8f70c9a62a126609167a597c3a1455670174eceea2ef5da27ae1",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "079c170bd843b3881b53da780c1112fdfcc3f785307c8c6b0d2894265066e80d",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "6768c0d7317b8acc3ca6723ff3c3ef489af7686dbb03573bcdd6cd50c0d5d056",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "6dec05e34ac9fb003579422451b8ea42bee3431e6205e5230506ece464fa6fff",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "51445e14ddcd52f4c22cbddc6d6b2600417bf3a7997b7b9194b625e5f155c1b3",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "12e990086e4fd43d4b49f948be30f7a78c53a24f92079129893147ab2bbea455",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "13186f31e39295c8b8bd6927166385db8e67ff6b444bbcb357502b4b3b144951",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "77b2e3f05d3e99afdf1136c43a5b983f9f9a935e121ceaf9f10c96b37fdfbb2e",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "07a195152e095e8ad17c2f0358fce460a5d19f30c024866bd598639c12ddb0a4",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "63b795fc7ad3255d1c7706d917a8f908bc8b61bf4f84f3cb296fa9c59c2ec4de",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "3e8fe83d032f0137afa1fd5dc541264390433b02cf8de43ba8368f02389e5fc8",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "22d60899a6258c86a59d5da51260cde3dfc51a8e33e0373108704c8de8efd13c",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "75ba3b797fac4007de1c5ae161bbfd8494f24270670845492f8b15b90570a294",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "0904d07b87779e5eb38847bceb40126060fe8a8b6c7d8a9a6239dbc070cdd196",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "6240aacebaf72a6c167697ada081f93106952f0cbd2d0c39f4322d6648f940b9",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "43e2143fbc1dde072c63cc63ad86cc51cf31db3ec74c8daab4ce1fd4ddba919c",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "56bdaf238db40cac66f13ba7e7c9316ad6947c5bca37d25af834749c5ba295a0",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "03aa31507e1e57547c9b8591d7a14f5c062a6bb7622386b91310d36cc19d3bb2",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "1d24a86d837413189e0e14521d5a5572338568d56eb93d40362ab9e3f53533eb",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "43b261dc9aeb485988d225821d09357ce045eaf054ac8c1cf4ec7648ffd4ce1f",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "02b4d5e242940964028d10ddd54f956794fe7126000bf47b19513d8b6c951364",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "326055cf5b276bc2b02c2ee2603dea33a09ed07dc17a359de55b1e1988bb79bb",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "27a6c809ae5d3410c49cf4936c824389eacc4646186ce508b4a155cb28d18df2",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "221503603d8c259979fa592469d7036cdd4a3e576a66cab2cd2c270ac43d6954",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2cb67174ff60a17ef22edfa315f5585a37d3d73a675a5be88ba6ebcd1f0db188",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "7b1df4b73890f43682891c667a94f0f4a9422044728ce3f159eecdf9390be1d0",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "68698245d352e03db2aaa88d1fb6a630e3555c9fd49409d45f2e221807f8f58c",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "61fcef2658fc599215eb8fe20d7f7b0e7c6c9e062b551160e492f2e0b3b2a224",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "46cf4c473daf01cf44bae2810ff6c482f3e4aad386ddacd7dbb15d852a18187a",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "1065aae50d8cc5bb2488c38c5629ceba7c1e7ef8392b4854213c6ea7f1498140",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "29387bcd14eb5f4072b1a7f2cbe5cadc0e5eda0116903303426525ed9ec4e5f9",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "69a198e64f1ce7160a07e7b1e18340305c3b2dd6bfca674a1c2c4525df200d57",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "25b2697bd267f9e470019576ed86a0e747ed5d963c84fb33e1014434dcc5caed",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "7369e6a92493a1bf9df54a66405070b847c9889cc85096679062b2e0d91a78bc",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "1420683db54e4cd2e04ecc3bdf273b5e3ca5fbd9415dc7b89d673ffb13986864",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "31e83b4161d081c1d3b0da49a66bde536a1b0ce99646ac8b34eebb6fc1cc5ad0",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "60b63bebf508a72dfb02d32fccbaac5c620c35005e58c102b478bd1e249dd197",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "55cf1eb62d5503175bece14b6f18683f49e48f4f29320ad897e8c7129e062b4f",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "1a56fbaa62ba0133bd831ce34913ee20d73ab9dde799cc363076b5e37df58c52",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "669a6564570891d4e2402fa912c55fa78b9d086d5094819c5879101065c23d58",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "13c4836799c58a5c9873ae5641347651302557bba77c371a943e6b505c9dc9ec",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "50bdc87dc8e5b827d4b2e883b8e55365deebc4ec571a4842c4dcfb6a5d8bd080",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "66f80c93a637b60719f83664ecb5b9b6fc13c187c7f13f61423a5d465ab3e1b9",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "1dd56444725fd5ae64b03ac325b73b9632353e15f011abd9606d37836edfe111",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "3a3a450f63a305cdf3e38be19fe7977c7d4cea11eae1c3e0c297e60008bac89a",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0e645912219f732e6e71454349220c8bbc9f6ac471cd7c158fa47ff83362127d",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "318c8d9393a9a87bd1e36c6d17996f80389d3183de94a510078f2f31d8394627",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "509a41c325950af655851dfdf35973cdfc921658342d9e3b5d669e29ab1dd398",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "224c7c679a1d5314bc98d3e3ba8598ef0c9f3c497f24db66f2745d032afffe19",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "05bff02328a1138982ec12809d833e89793ef3f4641b1f33bdc06edca6f925e9",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "576277cdee01a3ea1f748e6b8f4a52404fe70dc844a5fafe6881a0dd0dc512e4",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "780b8cc3fa2a44a796741049d21a1c88544acf0ad1accf593632137023cae00b",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "5ff418726271b7a15e82a51434e62a0d9a577fbd1405de081ef38abc234f305f",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "29d4db8ca0a0cb69ac1f26e938781276f35d2a3b432610e1e5db47e813b69540",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "106a03dc25a966befa98894c06bd035da7602025f3e778f5398e080c1789db9d",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4ab38a51052cbefa3c57658ac888f7f038669da5acd309e5d9ad0aaf333353d0",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "3041c6bb04ed2b9b25d56b5d201634c2e82b3efdf7575dced6cfd1ef5fddc09c",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "29cdd1adc088a690f1a80bd5ca0ace1d98c1c0574422ca13da7c2b256768d593",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "104bbd6814049a7b1a4698bb5f6c025cade797759f356b2e0ff2f2f9d956e148",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "204f2a20fb072df5b7f8024cde20f257e92be69d4cc75681a95d9a5fd67ff163",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "5be8cc57092a714944dc5c4304d2f2de2c811dcdd86f3bc251f0fd3168f1ed67",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "2570fb17c279161f653c3c318f6d5c317589155abd652e30c8143b3d30ebb079",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "241c5f91de0186687986ea2d88a4c935c8e6fba88f9050d1192ea9550bb8245a",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "3dcb65ea53030acde8c7142a65b52562f5f96f761cd6026c3efa367f2cb61575",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "08420edd5fcdf0e516d7fcdd235b01d18fbf2cf022d9733a28d8172940de6caa",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "04d654f321db889c5d9670c7ebb91521b6135b5a276e06850358c34e04f410ce",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0d9a53efbc1769fde3179617fc39e62b57e118d4e21a3e6ecdff20ab8362fa4a",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4a7a4f2618991ad71cb608173334a2922954deb68da5dd2d5e7dc116ddbdb5d5",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "4a96314223e0ee33dd84856486899ef293da8270718147f224c3b291af372a4b",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "11b50c4cddd31848cdfcf08500e011123df65f346b5c1b8ff4a718025fb15f95",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4f4bce4dce6bcc51773348b63d02b3f2738e177e9c1576d9a6e8274408a4ffd6",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "3042cee561595f3748eb409bf26b4fa6e456718fcaec231730e2616ec49d0b6f",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "73fcdd14b71c01e6d21a09d71cea3cf426ea725692f58a9ea71fce5ae2242584",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "389e740c9a9ce1d64cae76215f841a7c855ae36dbce2310a427e7079449bac41",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "25c425e5d626369065fc3eaba19b91ede55b0b3227919ce1c9bd78f6570eac28",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "61545379465a678845b3f07d62c6381b97500323e348d0ad64fcb3ae34dcb9ce",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "6539a089154847598c14f6264e8a6c773ef976278e0623083f3e06a6f1d7de6e",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "0f65320ef019bd9048a89fd736ca716919b2bc3c98424f8eddc4dbd414bb4a19",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "11c5e4aac5cd186c624e5ce8f9b99e33c150544125c46845e9d21f74c3d2f773",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4cabc7bdec33072a59a8af0dfaf2939a4f3fe6e3163b5181d486d1b1cafde0c6",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "7ee498165acb2021f2778bfccf65cc9dfe30a72ca1404d9fc08f788f3f78d289",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "639b93f0321c858217dbed2a764fa12ac748c4c03afe4738239e9624089c0a2e",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "0edf493c85b602a6e7d2aec2ae72fd192b2b90d4809074897bd508e39111a1c3",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "04c00c54d1dfa3691c8fcffacae6bedea090403ff7f5f8356767c4d284764113",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "403b92e3019d4fb44fe41d7422b67f07ea574f0febade20eaecc8158599b5a68",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "355bb12ab26176f4aee8bfad04c7d65771a0f35a1480eff84dc22f818b465cf8",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "7518eaf8e052ad8e6f077cbf3bae3f2ded90039db3ceaa11a301dac75a8c7318",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0f9b8132182ec3120a6bc50cfa05e785e5bd84d9eca3b0c3a71e64cc7493bbf4",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "47c3871bbb1755c41815a929c9b1d1d90f2d60bcf4383298a48859c41b7f6c32",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "08fea02ce8d48d5fe53754ea441ae8e062ecc4b0b3a299b0fbe65d50c85066b0",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "00b89b85764699dcf762c11a47c3c66bf805b17dc98c5d6e5144539771ec4f48",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "473829a74f75d537b514cfcd5d89d665c86445204b685d23824ddd7668deead0",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "6c668b4d44e4d39015257390cfe12fb464c2ddceef03588f23d9533aad3902c9",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2078684c4833c6b4355eef24ac47eb0ae63bd7d8b2618df082d2da754679c418",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "7411a6054f8a433fa96c65a78c8eed7bf76a0ab281273e973b48cf217a78820c",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "1ec9a872458c398f8455ecba1eef35f568713159f392a102579ae53d18b175b4",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "7c136574fb8134ffb34c712cdcc2e488044cdc75603af1154d659d32b99dc86d",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "3add88a5c7cd646057e7cc9bf19575619b81d7020bc882b4b8e6a4d400a2509b",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "61ba1131a406a7201d2ca22af2f66e3a8f7e35985ff659ec85c298d459393046",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "249929fccc879e7485530268beb6d18702dfef6cf66c1fbcab895770b635dcf2",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "5620310cbbd8ece77bf15a3e26783307023b6b6cba7ebd89a3d0a0f116959029",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "7ec846f3658cec4da6ec7311abb594dd40e8ff676c8f61936646b5f477e285d6",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "37676a2a4d9d97308f6d878fc3f41c22b9dbf806a51222f5528993434934d643",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "6daaf723399b9dd5214c8fcfa2989fb8130f1d776c01cd139b5e8f3f1da22ec7",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "2b5449e58eb3bbaa2b1e583b0810c5db29b743e8148be884583b04bfacad8ea2",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "1304e9e71627d55100c3e53145747299f7ea38548ebda0b85f3a7562eb3dbe47",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4468de2d7c2dd693da0fe1fff979c60a3c1bab3f8b48dd0b789814d26adc9cfe",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "5da6427e63247352d4a866c5657a772c21113531435d0c284b9ad8c6f86307ce",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "6cce7c6ffb44bd6393a5b6d6447f996233e6dc4c23ddc75451bb355e9419469e",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "58f29abfe79f2ca888ad8c388d59580fb9066ef7bbae1ff81a94c688deac22ca",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "6af93724185b48703643d056d50b3ab9b14ce538462c293c4b5a64bf710ecdf6",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "577629c4a7f41e36f0495b0bbe01598254036f9f377e76a5e90ecfab8de73e68",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "701f25bb0caec18f83e236233c33289fd2e036134b5589733220024509c6a888",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "720a5bc050955e5120f5b522ac4e60d6844a06e674bfdbe49d18f6d97cbec113",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2e0c92bfbdc40be961e3061ff4bca59cf700660e9e25a87dc3a8b0f8e4616ced",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "0e9b9cbb144ef0ec691417f35c229346e84e8b376242abfc0c3f09439b805a35",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "366d44191cfd3cde44a8f1bf1c68d791c9c3ab370a723fb98dee9bd55db1beee",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "221104eb3f337bd8d4813152635543bfee81916bdbf90d0efbbad48ffb5720ad",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4167a4e6bc593244ccb82f0e68a7fb972eda26fcb5856c3b9e3c1743f2bc8c14",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "34b33370cb7ed2f6f12e6e7e2f364eeee967ff14e880d62cc2be2665f8ce8fee",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "7938bb7e2255246a1b4b430321fc06845d1d9d400e7668eb643b9d2876f62700",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "1176fc6e2dfe65e4ed7485c158808883ce02109ced85a753cdc591ee8681d6cc",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "1f6a3e54f26906b621354ffeded7879b98fbcc2aacf440a3db90e28949770eb8",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "08fc3a4c677d5f343d4fa502ebe94dc42ddfc9f4b2a58480b4af6cd05b9c619b",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4f2fad0116b900d1e2333e23f759829540c085b631165cd660a4c199d30734ea",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "3451995f2944ee816f619b39f3b61689e60577aafc129c08962cd91db73bb638",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "445484a4972ef7aba61e6b2d368d04985f541c511857ef6c44beb24194ae4e54",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "10b89ca6042893b7258e9aaa47285c404a816c94b0935cf69152fcd09fea7d7c",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "17d2d1d9ef0d4da911776b9c72ff51b6d07caeed6d9c5f65753941be5a45f06e",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "20b878d577b7518e0262bfcb14c3ef1512ebf8c524533f263d5947499718289c",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "72214f63cc65c6a722e3b72c3ca60022fd3fe519d752106927f2af18073f3e6a",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "1f24ac855a1545b0f2c072bd312f9dc4d605824a4f518f751d9db7b9f43b29c9",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "5fdf48c58171cbc9d6fbd0a773761099aba714d72f336795b4e37f405307a693",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "00ba739e2ae395e6c7ffe45c06fb25a24748c1d10c1420ee24d608328e9505aa",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "72297de7d518d2265c9f030c26694e50360679d984973bfbae4426f5ea88bb26",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "03283a3e67ad78f3115a3b60f9b49922e5bfb7d345c2a2df592e98de5c8790d6",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "05e1296846271945d3dfc90d0228930832f19b4d8b63308048241dc7be0cb939",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "78cf25d38258ce4c843566a6f5c8df92bcc80cecd03081d9adbfbbc8242c4550",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "39c00c9c13698d9bb02dabae93b5d1e0ceefc8fcf12bb97cba82eeb32d9c495a",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "006b52076b3ff832c9a75a97f04efa05aa851cab9c2bf08715ae6b8e31489d68",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "7de2e9561a9f75ced4b36bce2bf4a7ab3407f14c417abc29f5cb7e16b9ce082d",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "4d57e3443bc7612257df39d370516b39b681df18966310e229e0cfe19d95781c",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "6b2a90af1a6029eddbc9c440d3ee9a814801527f5d85db99de70d4f4b6a55ecb",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "465812c8276c2109c647e6f24cee7333d8301b472fb9079b77ebf3245bb2d80a",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "5fd8f4e9d12d3e4aa764ae43e6edd12d5735281de03f5fd16923f4fc9ae61e97",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "05b32c2b1cb16790180d4a7bde2968d77065fb753831dc164d43beb22a1062d9",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "31771a48567307e1b6c29d0d340b979d3214286e4333f3ccf7fca42c7ad58195",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "3b5556a37b471e99dbbeeff27df9cd61a1cf1aac05dfef83c8c05eccd24da8fd",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4fc079d27a7336eba3d16048282b5af3edb351541a2ba4b632b0c524e14dd482",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "241170c2bae3cd086422f74d643e3cb91337cbc9cc94e651dc348b440c86c50d",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "124567cecaf98e92ffffc09c7880e4532497bd6502dfe9a751c938b089bf2f7f",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "3f8612966c87000d4ae75060ebc6c4aff0911dee0113e4353ff9ab860ac473b4",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "26f57eee878a19d4dc9f350b9f92d0aa29159db373899ddd9c18fcfa36048d13",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "504aa7767973613d7f62865b31ef238c551dcdb2ea718385559a0cc9782a0dde",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4041943d9dba306985c15a344f5a24675180d162247af17b0cab2cd55687efb1",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "12d931429800d019cb1d4f7a03fbc9e347a6b424648ab7ce4b217743a26caadd",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "1420a1d97684340fd9fa95ee1c77ccc68d749c9c26ea9cafc3c0eeba43ebcc96",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "1b4f92314359a01244182854e35ff3955e3c5140b23aa47b00c67799d337594f",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "5ff191d56f9a23f6ab82aa4051def4f6251f73d2215f485933cf3030a49866b1",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "239b572a7f132dae20eae43f975f302039cefa912de9696a3e5c109d89150951",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "020c526a758f36cbef4572805593eb3d2883ab795fc98523819ed433ac2d9068",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "452cfe0a5602c50cadc8e18aaec759972c589c9d8e124bb6e931ef59f042cc89",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "02aacc4615313877a9524cdca3e1b074c8f2aaf9dc7ca46c779834f89ed8dbbc",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4cfb7d7b304b877bab17ea25f1fb11c9bbc464270e607c9f86a0f7a0647877df",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "5b1d4cbc434d3ac5c343c857ecc970d02b6ecd71df57190de28699c29789ef12",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "14b4622b39075364b8c3aa373ee34c9f54c694d99c6adc8072b43d6cb89b75fe",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "2f98f71258592dd11301498b3369a7053a4f0e2bb88dcce5b6fb2615cc0a9f26",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "1d8062e9e7614554cebf890d75835de0fcfe3ef0a9cbd7de2e12ae444f54a701",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "3a6ae249d806aaf99a2acc2182300f675b1ff4a98e8e13200c94a74cb50f9e56",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "298b8ce8aef25ff38d0e1dfbdf34b4e91a0ea8b591b90f62657ada85a9907c5a",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "6ba6271803a7d7dc521636c77738ae703fab07b40bcf79f6837a72ea0a2165de",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "34b8a8404d5ce4859418457a30a7cf6a4b89c92a791570762a927953eff70cb2",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "71d62bdd465e1c6aa293aa9aa4b22573d5a813df63b5fefdc26eecb583693335",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "1c333621262b2b3d14571fea3f49f085d77f95cf16b065f5cd2db5dab1f75ef5",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "74d5f317f3172ba4e3645ff9f701da5af6db43790a0fa4b46533cc28d378df80",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4aebcc4547e9d98cda6d0892e3ac623b398b7c752b298c37a86fe55467d9ca81",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "173bd9ddc9a1df2cdbe63a034a58a207806b32535ba85b6e0b408d9e7354b610",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "5d88fbf95a3db7925308129b71d6fba9e7b8bac586c48c7012f0071b276d01c9",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "06e1cd13b19ea319e6ed278ec9673ae058d6582ed43918c12b500f1efe5872df",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "7f3a6a1a8d837b130c785f469643bf273baa0b90278d0447472baf629e5b0353",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "3080603526e162663fe35e14a04d088e118e32931fab6abe40d0ad516f166f23",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "30d0fded2e51307e68cd7830592c633995a8d555c901edf6f7e644395d3d800b",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "4615084351c589d95c8de72672fa412ba09572296664bbcf9cb4971e68b84750",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0a9214202c0998686965187f8f499a771bdbe78ef0cc4d9ce0594d1af21233b3",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "3bca0d2895ca5dfe0e6df501659932ec55c7110d16034caebc9019c0aeb9a02e",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "4ea8b4038df28241ca2d955f5f7a9fe2f0bc83ada644896f9c688eb69ecc01bf",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "5631deddae8f13cd98183da2130fb54519fc8b3ecff07a6040f031bc3c5d62a4",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "24ce0930542ca4639121774549f11a5f46305305a48cee832aed460af1cad202",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "549928a7324f4280b2e064de6492f844d2f7168e36372ea43fcfa155fdf30b85",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "41d4e3c28a06d74b827808fe6e8caf3eb5c468355d8810f21fe890f5fd06c106",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "491b66dec0dcff6abc3bd33bd17f4d69ae91e4b7d25ffdeaf26e32a763ee1a2e",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "4cf6b8b0b7018b678234a3791f7b7ba4ed222caf67e2284b75f04a8ed0da64a1",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "08f338d0c85ee4acac0abf52cbf8d947e3d5f8cc7e16db9898f5b13dc7ea32a7",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "530d4a82eb078a99c13d331b84777063ab27bc01df320c7ac383a821991a73bd",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "0db3e7e00cbfbd5b2bacf412c8cfb850257fb2fc4900a8806d6973456c9abf9e",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "65ea753f101770b9c7482323cc84ff8b7e2d78268cab535a004c3630e1f94825",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "02a4ec1921e1a1db0fbe044213443b1a81d62c7f61b5cb6b3d66fc3ee2096363",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "36d12b5dec067fcf172124851c063578118c861926ee57f2f5c86162f1cf795f",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "3d143c51127809bfcccbe6e88ba07037b8577acc45afa0b85ce6259a3b24b8a2",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "2ef517885ba8285936bdb6e8df179bacd5e48f5cfc783a0a126d279179154557",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "57968290bb3a0095f0f52dc746d5dd25d3f938ad889596b896eebffb305b2f51",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2f1b78fab143a8a6e185d956e980718ab9ef22fbabf041a44637974e8c58aedc",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "7dc43e35dc2aa3e1cf7509a86ee2eed1f393658d24f0ec47f71ab8430a20e101",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "044fb81d82d50a990f2ed8051f237d3e3576c6995e4efd945a782a5c273e9718",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "7ef72016758cc12fc6e08df8ef2079b1c90f9b314bb0535585966665887dd9c3",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "7f79823f9c30dd2eca704534b201bb4957b3371dce4c6359c1df18c5a907e3d9",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "64a9b0431c06d6f03cbf99557ded5be70827894e0050c8de6a9c1ff068f587ba",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0a4e0373d784d9b412b54136f590bd33c13670d4b91fa8d88334d239a3b513e8",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "071a7d0ace18346c7156ce4389a45d47b0b4f6a0d53a82352eb3d6a15b7d2919",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "7c69bcf7617755d39af5621b209d5f360d65950709b15141cc0c355220e14431",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2c3bcc7146ea7e9ccf543002c0ef768b01262905bfa562eed3072daac887ba0b",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "6f5a9a7322aca51de951a9a3171798d710db18252f50f37d07f0d7eb04e8295f",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "02ab9680ee8d3b244525567a47869c038d9e09408078af9ee729d4eba3d944be",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "494e21a2e147afca4efa47703cc51c9fc49f79c10cfefb9b8ba1000c2f41c6c5",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "6b5d76cbea46bb34fa091f1dd91ef6d9219a224e0fb9a249efa48a85dde50d9a",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "16fb869c03dd313e408b3ea2d0fcc746f1e6ae74036936d3e0f941171e782522",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0ae333f685277354af0169148f42b4776472dc6f5cd01dba8857556cec0cd994",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "3f81e38b8f70d0754811f7ed0991d03e24fc72b4d8abe133288e199733b60962",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "0ad3e2d34cdedc3dd57c3e8bcbf8eaf774b923c3d74299a40adb7f355f17c824",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "50510fc104f50993a8397ed24b0c4704545cb8a12465874b7f910fcc7ed9affe",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "127c158bf0fa1ebef2d6fd0009eefe1c745ede19c331cfd96f0c0fc5336e249d",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "4ed82479d167df956240680b873848a81d9973d3744dfe96dea28fc4ae51b974",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "354ef87d07ef4f689b413fc14b4eaccba44addd452ca3647f6197c422e9879a2",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "302d92d20539236d8808ac30a9f6653c50352efceb41b0b8fee3b52260c5d975",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "0edc5f5eb426d0605e1966d4787af6e804e1bf29a4bd6a932dbc6fb6e4e0f177",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "10e5d3b76f1cae4caec7bcb8c268fa86ed62f091a1863dd97813c1a2bca4283d",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "45b46c51361cba72bf87263b03578a23e9dc1eec24a9f6415453bfd653da8e67",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "5c1c0aef321229df4b594f7bb30e9958ab13645676620e30ce9d4ddd8a7fe3e4",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2177bfa36dcb713b1dbbd54b23a8be84e257f1dc8e8cf450a9402abf314f7fa1",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4ae619387d8ab5bb087a76659c8324876048811ec25f59b337081bbcfa79db8f",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "75685abe5ba43d6483ac3448d425904bfce0462a7196313661117e44985bfb83",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "1a00d91b17bf3e035eb0eb974a130d607d88eab4b41b40788ddbf6aa5344a32e",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "135529b623b0e6aadf7275107af66569543d0fa8c9ff49526e960933eb61f2b2",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "5972ea051590a613ec9da63714254aaeb42beb19e80985c1f5c716bce22e83fe",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "79b5b81a65ca3a018732e1f07114759b979f7888cfc11f1118f0dbd7add1d518",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4f7e9c95905f3bdbc01b2d64b33604349a9ad294ac4d4fa80fd4ac20dc8f7811",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "1942eec4a144adc88092499ef1a494668bcd3b1cdbebead771c8443d355299fe",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "31993ad92e638e4c8c2999ae53fbd9c6d8520f3989addc0f62674bbc5781302e",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "2a0a65314ef9ca75553ce494253c11222c1b3d910cea3e927dac5319ae234992",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "055dc39b6dea1a1360e860e9a8cda6ab2f9ebcac5a35bc3bcf361acd3c1c793a",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "4cf6e218647c2ded5982f3a21155af76db741f0617d0a6352db7937ff7f927c2",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "121a307710aa24b6a83c78cee4a32c8907e24ebc774dffabb119227cc28d5bb6",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "527bb94a6ced3a9b289e28231097bcd388bfe077b82b96afd659713ec77483c9",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "333fc76c7a40e52d460546919551d3b1e153fc093034bc2de4db5d5e9f034a97",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "6b89069b20a7a9f7485035de2f64d8e53405d07c6e383801563d992a995b482e",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "0473d308a7639bcf29e6c8d9f6e7a57e068686f8c734c1554082fa8cb5c7db77",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "23bc2103aa73eb73ffadc4ce5072ef05995a89faf9245b4e812aa0416270220d",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "102f73bfde04341a02a1ef74e601a94f2b4b421246dcc492caee792603589e05",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "3f6bd725da4ea12d981f375df55042347ac860292bffff06a2b4dae0b5511c9a",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "36597d25ea5c013db72712da2df7afa9023a8aee5787c690eb18b9ab7f5745c6",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "7b7ecc19da60d6d06466f8f99202932dd940579e6fc6905f734d8d7b106058ac",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "03bedc661bf5caba09bbffcd8f2d82db82263654e7a386506dae4a51a77cfa9b",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "4235ad7601743956951d44444ae12bd2dd252e660642906e78c2373c695c690d",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "0e7ce2b0cdf066a1a0cab423e2e36ee4492942549189f2986258cb0d078975f5",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "615256138aeceeb54882d47e7f2fab89f130c051c1fcba2dfea6fedfd94b70f9",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "666e0a5d8fb4674a09db17dd3ae94d48fd361df43c6139adc494643ac48c85a3",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "7f0bc810d514dee49abe4eba75e8985dcd65bcf0aa458b6b2abbf64e4870cb0d",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "1d3a907ddec5ab75311e2edd43ec69579ff6f8ba2ef72e9883ac9dad737213a0",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0d1f8dbcf8eedbf5cbc8dfd94f463c288d67369e57e03035b9006ba426f4136f",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "00011b44a31bfde30128013c030321cb29329fad851b3480ba1693313ed081dc",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "63d988a2d285102611a8dd7f9a7966adc1bf725c5852bd6a16561f696a0aa75c",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "3ac48d916e8357e17ae38b38268e4d715d40e38e4dd60dd23fdfa06c3fc66c0c",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "33fad52b2368a066f81669b384e72b91e92bceb8fdd8f68300120753afbd232e",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "3cc355ccb90a71e2a3feb6e6ecf6a56f072b4f7b05a13acb8d2cc8d0c422cfe8",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "16c0f429a256dca7b2acfcd2f305e7460af86430333f7735540649c6c5e41e16",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "631eaf426bae7568c87cd1a4baba9244b8a494cb7a5637cee9b69443903e9131",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "431f2c7f665f80b553658f2732e45de17280c5fbe2f8055247d975b9a3700de8",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "5599648b1ea919b5bc3d97611ef9bf8385dd4b526c16e5a6b3e90410da66fe9f",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "7b04715f91253b268900441a2090a9d714ab352fa1ea514ad6026344858f7b19",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "48d0acfa57cde223b09a9558450bf944970ed3dd6d1d9b0bb376c280c4e6bac6",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "59b37bf5c2f6583fc0404769b7eb2c4486357c8b7d5c7ab483edbd28acf6ae43",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "16234191336d3bdb4208ce7ee9960394f1d1a197622f3a37b60f26e47dabe671",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "4a25707a203b923151e2d8023dd73dc329cd9bc3063625a0dd499cd61ff38640",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2cdfdfecd5d0500623a0153fe9a4f2b17772ca7b742c0843b9e499def6267ff6",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "5721896d248e49fc4000144c3ae20161304242581dd170a12ab7668a53f6ed6a",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "03c935afc4b030fd63e5177ce19393b34baa6fa7b5fe3e08285d5091a1d0da4e",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "578edd74f63c13da5509bce9320646259022629f2bb963b40b6e5517fd181bae",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "3ec2ab590288c7a2dcd29b84dd623a3c47ccc2c4dfe205fc997276c6492b0c3d",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "0e1bf66c6adbac5e0baea4c6e81eab290f2b87df40f5c2d5a7213a09ae32d1cb",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2dd5c25a200fcace99a0ddd073cb9b83a98b4deb61391aeda1a0d27be4d87bb9",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "730548b35ae88f5fbfba69cdbaae5f1e1a020018cb926d5de2abd5e9792c887e",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "585a2277d87845dd423f06cb0622702bbf3ef17709353f19805b094ba1d6e334",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "3043443b411db8ca760f4f52ab8c385065a26f1db2115f16c43551a3cba8b8ee",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "7656278950ef981fa78e6fa5373e41ff6698c4b5ec78257fa18a5f8233d48962",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "28bbf484b613f6164853e7fc31838a8e3a8cfbb707155fdce17073a3ea86cf9d",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "3f4160a8c1b846a626bf109fab570e8f9bedd2fd0506b6f238c3cf59d51fc8c0",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "1e79cb358188f75d527e9ad213de6f33afead107f6dd11bef2612f5c6f136c7c",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "478ab52d39d1f2f4dc6c2d0c864525e584a50c44299dded977e953d8f5e08181",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "65d7951b3a3b38317ff908e5bcf9defc828b6a7ffe9e10f8013436c3eef7e3f1",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "16d87a411a212214b82c6b40a6c1c96fe5dde1bc871ac80766a6a4d39252d159",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "1f990b577a5a6dde497ac2736ee9778fe21fafd72ebd99fafba4d5e2d54e0583",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "28f87c8165f38ca657c05db1d6f994b7879be3cd0c5a24c1b3bd7e5a42066215",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "45882fe1534d6d3e77c6569e520de0527d1e50ebacea798fa3344ead1be8f7d6",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "7dcf843ce405f0742699db13bec89af3b5f9f161a38392a2d8ac9929943c6fe4",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "688fe5b8f626f6dd7ddd1857985e128f62b6ed1117aa11a66669345d757983d6",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "6739687e7327191bb3be28c3915dc6e1d52143fdca5632996c90d6484a4732c0",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "7da0b8f68d7e7daba7ebf3be95dbd7c697a05cf41b38a436a66dcc9dc80c1ac0",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "46115aba1d4dc0b34cd1eb505cdfa8cba2649f30aafda9e8ef782014385675a6",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "5a5f887e83674b4b03cc6021feb259601dac6f7321119e9bd40f1953c3b5da76",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "43e37ae2d5d1c70c9b5302897c2dec32b5c3cb00e6c320649e9628d3a0a643b9",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "3669b656e44d1434f70297d4a4bca47ecfceb815350dd0c48f6301cf70a13d11",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "6a21e6cd4fd5e9bebd5ad8f83626381167301d5199a13ac0387e3f06eda6e133",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "45371b07001e8b36325432d01182b0bd71d30847708d1301ef4129126699b2e3",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "586bf9f1a195ff5769dbbd3c8c82b75558712a2a00d23524f1c6170a3046e65f",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "0eafb03790e52179ac0349d261a16eb85278f0dc610937e5a6db088d5ef8790b",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "44ae3344c5435bb32cebdf1eea92396dec02fbe32662cc305140805e0f75ae1d",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "309acc675a02c6611c81f73873486d0c219a41e6820baa11960555c13748042f",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "62d5221b7f94678f1d82e5c64f9360aaf3760e9d5ac971429cf289b9bba543ee",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "14f29a5383922037a506708059f7193ddfae7b11fee9144d7585d4263af77a3c",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "5c0efde4bc754562a375052edb4a8631c86bb56c8a0c1a0c524c299c18d0936d",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "5e72365c7bee093eda9234b7c3ed4c6221f970db99b53040df717edc25b2d7f5",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "70b20555cb7349b7acff3dad1f9ebdfd5b9659e5df9f32be7d9339062f08b33e",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "77f1104c47b4eabc9a0a37bbf4191e333779675d0694d95b575bfc074571217f",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "0e34398f4a06404a446677855e503b476688423a9a881fcdbe5113c555112c4c",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "6f8aded6ca379c3ef43217da73395d6f7de3e10e73f3f64018930b093e4b1928",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "5e5405368a362372743fa61fb05b6d8d09b3e84127822f07b67d22d93ecebde8",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "780de72ec8d3de97f9967d02fde6949e487b97e1a21ab291e340123dfdb7b29a",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "1defc6ad32b587a629a17fd7973732928f72eb2a2a8c41aa671feaf300f42772",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0eb28bf671928ce438ac15510a4811b8388ddecf1c7f4d060ae28545089ae7bc",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "467d201bf8dd28672991f7fb7ae5da2e148c1277917b15edaf5bbe1aef5195a7",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "2ef9c5a5ba384001e41064d22c1f4ec83f624cb2d64498bdbc1ef4bd567ae7a9",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "71c43c4f5ba3c797964e01d309a47b013a827becf6a308a295fe919a74ef4fad",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "5a758ca390c5f2938255b3d0f1ef990ef18278bce4af267ab6fd6df6fa9e74cd",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "0a738027f639d43fd4e6a829afe8aad38ded36469a8130668ce0918b1d61dc94",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2dbae244b3eb72ece3d400bfa0b487ca3aa8c6d2d57d5003a2b72710d9462495",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "2698ca635126a69c105c3f4a49fb15fd00670d0de1839843980f4a2f57ffe1cc",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "3c004b0c4afa33325e773ef6024da96a9e3f0918e4d253862e3d702f5e3dd90e",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "1fb43dcc49caeb0708a81b91a0291fcc381831f7925cff8be765318832b0ba78",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "6b43fd01cd1fd2173ed3265fc6cd47871ca284a5a806c4f39aa946ac06f4b82b",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "73420b2d6ff0d9f0bf1427c2072b923f75dc52b9ee0ab990b5c742583e760ef3",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "5a68d7105b52f7142868b9ebaa46785a15fdf848df0fffbfc7a75d4b4697c544",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "3db5632fea34bc9eda8ab89699fbf3738f593913c62238c4af2cf6cb9e851e06",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "59197ea495df2b0eeef03d394c704af8edeaeb873e9a89912e4990b1829825d5",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "04321adf49d75f131bf2d131499e72730d17b1f6396759a5f46eee2bf75dd9d8",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "45eafdc1f4d70cc0c7ce2dc16f159aa4e77b437a7e2f92e904e16019e4e55aae",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "3f73ceea5d56d94031a09d1ddc0481c959dbc292bd5c0395b60e4624cfccb1ed",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "5a5eebc80362dade941a36656b222dc64c22faa2cf2f0651698401858045d72b",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "26058891266218db60c1207f1557aefabe57007e44c9b339b7a7bfd10a4e8dc6",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "0dedfa10b24443b8ec07cccab4129f085e422c9303ceccad4c818e3cc676e542",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "54ad0c2e4e615d57831b2a7312873551c3d93fde7661e6f459f704a68360ff04",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "62ecb2baa77a9408a5b4d2f26ec19fd336f163469fa5c1ebee3b67d5b82b522a",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "61913d5075663f985aad01adc630a14a5fcd5e8579e104a592072836afb62874",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2e59f919a966d8be7482c8d0b96b4c714962357d0eddd7d1e5ed795261152b3d",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "3ddbc2a131c05d5c02d801513f9594cefa475832942002700dc62d361a3231da",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "5ee7fb38d83205f9e54be21831a0391c883abab79b07da21f3aa57a22796bb14",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "78968314ac04b36b028007c7f0f89515039c2a6b8c2f130d9adc0ff9ce5ec54b",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "61d0633c9bca0d0946af908d263c8c78a5acfda9434937f9538dfdcb41446a8e",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "5b23ac2df8067bdc637fb4db38c2a909ee84695da6f037fcada328bcf8fc73df",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "6e965fd847aed7f56f1b3280553eec03c5bd6b89780b68bb63744935ffdb2566",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "79b9bbb9dd95dedc0e711704150e82cfe88f19aafade6d8d9ad2b953ee80527b",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "1ba811460accb834cd6cba126d445f0aa032a2f8cfbb0816d1997dae8e9f7374",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "5065f158c9fd21476c0c6429e5b97a82d26383a868c8c393ebb355406a3126c2",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "3981f39e58a4faf22eaab98a70e645bae14600acd76ecf67708169fb0c429954",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "30f4452edcbc1b65e9d2e163c7b4f632e152a5002c40483ac845dfa56de66fde",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "38dc083705acd0fd3a85a94514a93cb51d168f6960e6f45d18fb8a7559230a93",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "632d9a1a593f24698844fc73c0ea4e71fa134569f99cbecc856d2782c5759740",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "3a5a7df28af64ba221908c2d57cf877963f071810d9f693abf09fd11ed0c84a7",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0b24f48847ed4a57bb1d97036e29670b1823c7dfbc54f0d7f6bb6b15b807cba6",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "34fcf74475481f63e19cff9f005f9a65a4538075ed26ccf2dcdad4be511beac7",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "0119a3042fb374b09309c9110a92608e5ceda267190b72f2a5bb1dab78cfaa98",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "3d3bdc164dfa63f755de888283f95fa8b8714dcb38d9467dc197e04c789767ca",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "56c088f1ede20a73f56598e5b282a2b0669da5f66895d0c167a2d89ce8c2177d",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "038b7ea48359038f9a2169028acf92f0a90be9febae30cbd581b5fac24f38f02",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "09674c6b99108b87f915337625072988d7f388320b75b2fa336d3d1110a86e17",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "6e5e31025969eb650971a5ab5aef31742f49d282eaa78d4f9f4ef82199316ff8",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "3058ad43d1838620bd1924778c1061a3fb35068987acba3f3304fb0e63066222",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "08f5114789a8dba8b491c1e014cc3e6d4999eddeca5d3e71b16c62f587e593fb",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "26549fa4efe3dc99bdc78abdac994f9a05c3df38a22ea610323c0ffde57663d0",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "3a10dfe132ce3c853df23ff7a4ba0c47d77fcf04f14a0ea5db468549af3f666e",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "1926e1dc6401e0ffd45574a26474d3d92305b3fc7777a581741d5a461e6bf9d6",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "1fa1d01d861e5d15175322fd31f2c0f12fd515463a1fc1fde07f4e8aea17cea0",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "73fced18ee9a01e57f13e93efdd5e2622e712bddd1080de938dcac00d1df94ab",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "773563bc6a75cf3399f6f7744e0593201e4656da37f15520cc8055947d599832",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "1f426b701b864f448d77cec8ad638932a493da67c5a03ecd06b1e90863139cb3",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "0b76bb1b3fa7e438fa83406f0d9b723eb76b8153575e9c76f17e35c891a12552",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "4e1af5271d31b0905875da6bf30f1447f1a3b7b817a22c25efc9264c41911c01",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "7b6dd61eb772a95522e5646399bb8017be6771cbd444ab6e08b8c1f97f92939b",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "53fa9b659bff6afe866cbe65a0cbb28116fb76dc40143b185730abf9ab01d2c7",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "703e9bceaf1d2f4fad962dbd8dfc5bdb7998fa4f608cd5cfb7adc1e850f33d92",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "398d93e5c4c61f50181bb73ebcd65af1843a5d6665aed4e56c14c8e994885455",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "699c9c9002c30577283e26e7739ef1383b34aaa030828bb1c3877c60d2e7e3f2",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "53b09b5ddf191b13d43f8cf0a10b0376bd9e128715bf0a5f1c4bd16733e248f3",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "51caf30c6fcdd90728cdd24781b4e975921718b5cce5d97df306a7235946f1cc",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "12ae29c189f8e99a2b89bc334ce10cc7903378dcc51cb30f737af99a18ac54c7",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "5504aa292383fdaa3758563dcf3324cc630e8570a17a7bf3a60ba7427674e00a",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "5fd14fe958eba5ea55ca7521d09c4e220dd1efcc3a34f7aea99ec0cb1f0d01cf",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "7dd73f960725d128e0f0859e884220e8bedfa85136d4565f3c42fe5ebf93cb8e",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "093ff26e586474d14daaf3d64002e346069491b10a7fe993b5dc2ddf2845ab2c",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "2f59bcbc86b470a41367253ab457ac2975730672dbaf23e5b10d24fe68059829",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "2aa55e3d010828b116c2e1631133558585201b3fadd7e71e7041d560b691c301",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "5458b42e2e51af4a7e7748d9be77aad1ad1b911f567d03d783847d429917135f",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "13e9004a8a768664352b4c82fdb5c86442c54e2d74421d10ed5192e60c07444f",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "5f46040898de9e1aecf3b7c036de649fece3a890e0dc506bbb2e00c9193b877f",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "73937e8814bce45e32bc0dcab74ffef7fa38d6c9ae6bf863739d8845832fcedb",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "2cf8a4e891d5e835e19715574696bdc6a9d13b22d4f06834b9037116297bf48d",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "7a6f7f2891d6a4f6d7659c8185978a3024d2381c3950196b2cb5487e17d06ba2",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2049bd6e58252a097cb16a4cc2736a86dd4c09d37c38b5496d93fd8707110f67",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "6011aadfc545941d4c21b52c519ebfd4f0ee60be5b3db90b7d09fd8d6a9aef49",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "69ce18b779340b1e624d0afdb9b6ed99fbd098ca0dff6aaa63ded0c802cbf890",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "6fc5cc1b0b62f9e0d6a1e7f3998f7a5b7c7e8561712890715f67926dcf95f83c",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "15596b3ae57101f1127e0442189f2352dd1aae3cd47e9092d1ef5528b29879cb",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "3aa4e241afb6d138292b7d227ef556e50be4158bd9c745df09ff31167e5124ca",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0d2237687b5f4dda1307deb553f2148aff83123197d6ddcf462739d23f9179a2",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "32fcaa6e4687a36c083ab1a25549d2eb48583f8fa2e926c32cc138bf2a3305f5",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "746f6336c2600be9d5b2ecd7f60d964e17e31908f213e3f83207a4732787ccdf",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "4627e9cefebd6a5caac014de22efe4a33e0bd2ed9df0bdf27bc56e8dc57d9af5",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "7b4917007ed6629333596a8a0344186de288eb729943731f3f4af345ab6c971c",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "1d03620fe08cd37d0ff592d94dd2f8f4aa17905bdf42fc3f54341b28dd53a2dd",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "122ba376f844fd7b476adc447b15ce0c497810d289f3bc142d85fb5cab84b064",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "02c70bf52b68ec4c2eabb4be7dd479d99ed0fd42115d187fc20232cda2b4e554",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "6b0697e3feb43a0356cf7d94ee8bbde75be768e07cb73cb5ace532bf458d72e1",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "26334f0a409ef5e0e044a61ec1d0815c415c5790074882caa287ec4b5d0b2fbb",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "1cc37f43441b2fae9c9cb95849f0d2a93ef000ef076da45db6c8f04adf62a3c0",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "3a346f772f19683826e72832844c23341c5b15f818e5656ad76656f1c9ceaeb9",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "6c6809c10380314afb18abdb5c45ac19d061c4c0e506a922508f565a5cc7324f",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "05911b9f6ef7c5d0960746dd8ec67262e9bd0331b1e851edd2d55112e2da6ac8",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "555ef9d1c64b2d1724180c07a99a688d20c141d31cc1cb495349acf3512eeaef",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "6b3cecf9aa270c622cf1130a0bb398e1c0f3758f512c4cacc1339983f5df0ebb",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "675f4de13381749d5737ff98b40946f2624aef08a3afbf0c36a770ba3b73bd08",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "60f450b882cd3c43019c4ff39abe94870725d80f9d652dfea12ff6d93bdab31d",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "13d99df70164c520d0dcbdd96964073e64816c816cad83b40e2c52036b1782fc",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "1c06de9e55edac804e5f1c18750023a04fcb69c9d719bfa2014b5ec321e5c0ca",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "78096f8e7d32ba2d5e4a5c2fa34d989734530b18dc4049bbffd52b40ff6d69aa",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "4cf38a1fec2db60d9ca143c5632845156608f938be2ee08e990f7ad6a33ec4e2",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "373cad3a26091abe4f09cc7d7003725bf9c49e2a48b5478ca0aaaa650dfa5ce7",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "21547eda5112a6868f58a7bb1f9b8d9d3bcb2cbc61aeaecbf1bea8fb89ddbbad",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0040f3d9454419fc9e9c4db3418cdb5a1fcbfde124934536b294634d82c9f57c",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "63550a334a254df4b72ba407bb3119b9f4272c89510a380cdefde939fd5986d3",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "39a92bafaa7d767a60e8fa69c734f18df305c6fae2c408e09bba584572547b49",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "3eea62c7daf78f51ea7c0090eb6bd65d178429b00ca52ee16507d6edb569cf37",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "68436a0665c9c4cd70525560eb8ab39a5f63857768dbd3759d24c713e693274e",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "5373669c91611472c1e0a1c6320ffc7ac5266844840ae9651e56d317e820107c",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "17b6e7f68ab789f991a4e9d5a38c2416c75c00e264f975b0bc0235e8202f3f27",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "1cb4b5a678f87d11afcaf5885b2d1eca908f2084c9cab3fc5d2814ab9a0e5257",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "2dc0fd2d82ef6eac5707e438000be223f311e0b0f071c7b16b74aa62a2a007e7",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "330bca78de7434a24f8d03164bcad8340c88de2498da5fb1b664c06b394afc6c",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "3c31be1b369f1cf5c58ac14fbfc953fbf9695e962b074724982eff841119744e",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "41cec1097e7d55bb7f0e52aa34ac8d7aaeb8711fc7cedb98c168bc93f9cb4272",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "330b61134262d4bb7975cdaea6aca15807dc19ee91ba1c6fb0f4864d08948aee",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "26b44cd91f28777d73d7c36cdba1df27bb1fd49e1d9e156df79619d7a26d808a",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "7deb10149c72610fdafc778abfd413144b5279ffebca8a2ce1b7f29362730383",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2c709e6c1c10a5d69aab1244d99f2055b25dbcf49cbecb3c51f048478f387475",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "08e68e9ff62ce2ea588001380f0be4b566cbec045553cd0ecb62af6a8766ee7a",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "4cb13bd738f719f54a8f342673766cb9acb9218dc55923df2f2d09d50ab8f2f9",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2f1f3f87eeba3b09a25c3d98500a89be8d38db493d0bd49c34ad500a4bc130ad",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "19a1e353c0ae2ee8c20d313f3f751b50a59501badb4a9038f7848c75e515b64a",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "736bd3990266ae349fb15347b44109b593e0454398eefc40b42172cdd596bdbd",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "41546b11c20c3486e3529718c0d66473b3e1a0a0c6e55e617d1c7560bafa05c3",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "412295a2b87fab5ccc5f5f30425c837546fd114b6081657385532d509334b3b4",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "0760bb8d195add80460975cb7900996b845a92032133acdb2e655261e293eac6",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "14e153ebb52d2e345cee3213b30ad2735393cb266df8c82519c99b88f57ed6e9",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "6b34be9bc33ac5f22cbf268f46caccb157156da9ed69a084413e1a17cde6818a",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "01b9c7b62e6dd30be33ae7a2d4fe5035c6c9e845530e737a11fc69656571f2d3",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2012c18f0922dd2dec7d1c5e49c1b5a34c3e971ef22e027cf3df2f643a78c0b2",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "08af5b784e4bade83d36efdfc2e76c1f1483241f45a0a763880b55e55ac89d29",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "6423c1d5afd9826e18d528d6fa3364ce4be4bd11a287178de27314d289cc2c4b",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "32b79d71163a168d897addfb673441f49d0525da779323b6283499dc881f2533",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "480a5efbc13e2dcce49de338f9e5d3cd22bcc28f3746e5f9cc85f8d9edfcb36a",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "24b31d47691c8e06663fb4a4dc1cbe036e199dcc4c053928b6614ce442ce221f",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "47aa27600cb7bf312c6bb061ef5aabcd06b505cf8b1dafc50b51e70b01622071",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "545b585d14dda094f0af66134ea278d611a4fe7e7c693f7c2a541eedc015f8c3",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "4d22dc3e64c8d0710b0ccffd5b99bd9e3baa637a28ff1e956204e4d0e3b321e1",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0f62f9c332ba54544289134cd479e72eade68e34089beebe67bf275ea0d43a0f",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "1c0fa01a36371436febac2d2953afa055cae6a3f57cbcf61fcb46589d63b5f39",
            )),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "4005419b1d2bc1406fb73e54c4dfb634f9c9a035c1dac5b669082b0e8c936a50",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0fa3614f3b1ca6bf5d254ff397808678bc8cbece44cfb3a0d2c604b622943dff",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "1fba218aef40ab42f8499f911954fa8e2089c1af3a44ac90a003febdb9be82f0",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "6c535d13ff7761d5c839c6ab99dcdef1a81d3eee08daaf7f4f3e57043e7b0194",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "392e3acaa8c86e4274fd6c7d6c2b5e0181f6e89a7ba63741ab549448fac8f53e",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "25e17fe4d50ac13b19319c76f29ab0ab2e0781445887e8164cbd34e93e8a35af",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "6bba828f8969899bccba2fde4d1be526c34a32272fcd87e3915f7ff576f121a7",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "255751442a9fb3515186d8b034691fab208e1c52d6420f950a289bd71e04f676",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "559d504f7f6b7be4971d6914af8460d44cb54a18a0997ad5e2d1bc6690fe3901",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "123ea6a3354bd50e88388f1d6061e23b0744a19b0307781b9c4891e7f6d266fd",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "01ab12d5807e32170a26cf62f92800ba1d69d366a5553c7ca7738378b3eb54d5",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4a3961e2c918a1541eab4271d83245d9b9ede3c2d8315848118d189041e32d96",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "68aceead9bd4111f83b5a716f2dab979499a260e34fcf0160327d644f3233f1e",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2cd6bce3fb1db763a992425fe13a476ad6cef8347effe30a71dc3be0f8e6bba0",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "595760d5b508f5973860d9f1b7e73e23308e6e24b7ad040c38b4c90ef3d7c210",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "2cb2c5df54515a2b65f492e37d3473f489af3305c4115760882acbebfd022790",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "709fa43edcb291a93f8bc0897d9094588f960008a4a7fccb6129bfe104aa6397",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "70f029ecf0c8131f2723f36ef8cbb03ad22bc1662e694effeb0a5d8c63fd2aca",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "44eca5a2a74bd9e1773efb77aa3ad61f78f0a370ef041aa92a6aafaa5e10b0b9",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0b7d5d8a6c314858c94449d3195f0366ae042f33a45581e7461307b32eed3e33",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "272224512c7de9e4de1c531c60e1c52b70d38300a3340f1d25d448327b95d543",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "19735fd7f6bc20a6e27fc76fcdedaa488c5c397796ada358bf7bbb8a42a975fc",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "11b5df18a44cc543efa28c8dfcc84e29ffeed811b2e6fad01abc92af49c5342e",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "0fef911191df895f2503a1d065a497b9eb848e0f7f19547ee3ab90d042c84266",
            )),
        },
    ]),
]);

/// Odd multiples of the basepoint `[B, 3B, 5B, 7B, 9B, 11B, 13B, 15B, ..., 127B]`.
#[cfg(feature = "precomputed-tables")]
#[allow(dead_code)]
pub(crate) const AFFINE_ODD_MULTIPLES_OF_BASEPOINT: NafLookupTable8<AffineNielsPoint> =
    NafLookupTable8([
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "07cf9d3a33d4ba65270b4898643d42c2cf932dc6fb8c0e192fbc93c6f58c3b85",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "44fd2f9298f81267a5c18434688f8a09fd399f05d140beb39d103905d740913e",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "6f117b689f0c65a85a1b7dcbdd43598c26d9e823ccaac49eabc91205877aaa68",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "7a164e1b9a80f8f4c11b50029f016732025a8430e8864b8aaf25b0a84cee9730",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2ab91587555bda628131f31a214bd6bd3bd353fde5c1ba7d56611fe8a4fcd265",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "5a2826af12b9b4c6d170e5458cf2db4c589423221c35da6214ae933f0dd0d889",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "2945ccf146e206ebdd1beb0c5abfec448d5048c3c75eed02a212bc4408a5bb33",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "154a7e73eb1b55f3e33cf11cb864a087d50014d14b2729b77f9182c3a447d6ba",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "43aabe696b3bb69ab41b670b1bbda72d270e0807d0bdd1fcbcbbdbf1812a8285",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "461bea69283c927e71b2528228542e497470353ab39dc0d26b1a5cd0944ea3bf",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "1d6edd5d2e5317e09dea764f92192c3a6ca021533bba23a7ba6f2c9aaa3221b1",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "7a9fbb1c6a0f90a7529c41ba5877adf3b3035f47053ea49af1836dc801b8b3a2",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "34b9ed338add7f59ceb233c9c686f5b5a6509e6f51bc46c59b2e678aa6a8632f",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "49c05a51fadc9c8f96cbc608e75eb04498a081b6f520419bf36e217e039d8064",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "73c172021b008b06aaf6fc2993d4cf16e2ff83e8a719d22f06b4e8bf9045af1b",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "4275aae2546d8faf113e847117703406e5d9fecf02302e272fbf00848a802ade",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "18ab598029d5c77fa3a075556a8deb953ed6b36977088381315f5b0249864348",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "3dc65522b53df94844311199b51a8622031eb4a13282e4a4d82b2cc5fd6089e9",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "234fd7eec346f241537a0e12fb07ba07bf84b39ab5bcdedbbf70c222a2007f6d",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0267882d176024a79d12b232aaad5968aefcebc99b776f6b506f013b327fbf93",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "497ba6fdaa097863a2ef37f891a7e5332437e6b1df8dd4715360a119732ea378",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "61e22917f12de72b2dbdbdfac1f2d4d08648c28d189c246d24cecc0313cfeaa0",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "43b5cd4218d05ebf7508300807b25192d3829ba42a9910d6040bcd86468ccf0b",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "511d61210ae4d842032e5a7d93d64270eb38af4e373fdeee5d9a762f9bd0b516",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "6d325924ddb855e3aa9b36646f8f1248a54620cdc0d7044f92c676ef950e9d81",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "71a7fe6fe248281039fa4e2729942d258a1cf016b592edb4081386484420de87",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4746c4b6559eeaa972cf591883778d0c33fd1479fe5f2a036c7182b8a5c8c854",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "5c9a51de34fe9fb745651cf7b53a16b5defab2276f89f617d3777b3c6dc69a2b",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "219663497db5e6d6ff939a760672a3327d35aedd0efcc849348546c864741147",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4804503c608223bb09c3a71710142277ffdddaa1e658515bf510f1cf79f10e67",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "553398a51650696d88a96ed7c96e0e23a059a0e3a615acabc4249ed02ca37fc7",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "771e098858de4c5e5d9e5ce420838a47bbb40aa7e99b9e323b6821d23a36d175",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "5a5ed1d68ff5a611477f4a2d9fa595083ada5d7985899ccb9a12f5d278451edf",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "44acb897d8bf92f07387f8291e711e20cf209a257e4b35d81195122afe150e83",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "019b60135fefdc4428653c1eda1cabe9392e5c19cadb9d7ebae5e0c558527359",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "150c49fde6ad2f92506e88a8fc1a3ed7c4f5e64f24304c161e6068145e134b83",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "6b2b5a075bb9992210af79c425a708ad5d6fef394f75a6518e7bf29509471138",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "78a6d7791e05fbc1fe3ee3560c36168dc83f44dbb8714ad0b849863c9cdca868",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "725c7ffc4ad55d00aa2b1fb1d542f590a601b355741748d558bf704b47a0b976",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "794cc9277cb1f3a323d1157b8b12109f7352d51102a20d34e4426715d1cf99b2",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "4cd54625f855fae7df585d714902994cfe416ca4ed5e636691802bf71cd098c0",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "7008357b6fcc8e852ad032f10a311021bc9aedad32f672584af6c426c2ac5053",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "06ef7e9851ad0f6a8d2dd5a3b9ad29b6b8ccc8fa95fbccfb0b88672738773f01",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "13a92a3669d6d428b631639c4853620247ab6463d2b4792bd01b9fbb82584a34",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "3c296ddf8a2af86a24680f01d802e0717540e41e5035dc5cca93771cc0577de5",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "7a99d393490c77baaff823179f53d730a92f7bf98c8ff912aead15f9d914a713",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0a892c700747717bfc71a37dd0a1ad05b89510c740adb91ffceb4d2ebb1f2541",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "286762d28302f7d2a5a96563262f9ce077a8c84157e807948f52ed2436bda3e8",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "32f1da046a9d9d3a0cc192d3cbc7b83f82e1181db26baa974e7836093ce35b25",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "7686f2a3d6f1767a154a179f3bbb89b8e4986cb46747bc637c558e2bce2ef5bd",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "561305f8a9ad28a63f91dc73c209b0228f11930304d3852baa8d12a66d597c6a",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "00aaec53e35d4d2c83131b22d847ba48ca43d5434d6d73e5100c978dec92aed1",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "030a1aef2c57cc6ccaf68da7d7010a61709de9bbdb075c536722cc28e7b0c0d5",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "20be9c1c240654807821dc86520ed23e0b3f29802b2166087bb1f773003ad2aa",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "750ab3361f0ac1de1c7f9a81c36f63b55943bc2df546e493e15387d8249673a6",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "69764c545067e311105d639cf95a0d1cb03b3b2fcbdcb93820e0e44ae2025e60",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "54f96b3fb7075040b72fd15bac2e25636f2eda23bd7fcbf11e8a3283a2f81037",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "3d7fe9c52bb7539e6f3d94828c5760f13adda2047d7d724a0fadf20429669279",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "78e6839fbe85dbf0b7a8a110e6ece78589764b9cfa576479177dafc616b11ecd",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "70f268f350088242320ff74aa0e59e2275d05d43041a178a70332df737b8856b",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "7c0d345cfad889d9e9874eb71e4cb006f535c5d160dd7c1966864583b1805f47",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "404e56c039b8c4b9b31ddeed3552b698380cc97ee7fce1172324112070dcf355",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "20d754762aaec7775cbc4152b45f3d44a0366ab167e0b5e1591f1f4b8c78338a",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "2b8f1e46f3ceec62a9934a7d903bc922ace543a5363cbb9a5e8fc36fc73bb758",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "38b8ada8790b4ce1e9322b0757138ba984b37df1de8c956c9d74feb135b9f543",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "049aeb32ceba19531d106d8b328b66da2b3952aecb1fdeacb5c04a9cdf51f95d",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "795ee17530f674fc1d82542b396b39300fef924b7a6725d3aa507d0b75fc7931",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "51c665e0c8d625fcb6676861e14f7c13209c594897856e40d7767d3c63dcfe7e",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "6cd19bf49dc54477e6a24d0dcaee4a315d411f6ee034afe7254a5b0a52ecbd81",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "5e01b3a7429e43e776f6627e20990baa082a2a88b8d51b101ffe612165afc386",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "7520f3db40b2e63880a2baa88499711e571d0a060b2c9f857e87619052179ca3",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "71092c9ccef3c9861a309a64df311e6e967b6cdd599e94a53db50be3d39357a1",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "4173a5bb7137abde3a4ae7cbc9743ceb03f6a40855b7aa1e856bd8ac74051dcf",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0ea15b03d0257ba7080b4a9e6681e5a4a2b404f43fab6b1c53d8523f0364918c",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "5fc565614a4e3a67f7931668f4b2f1765a696e2b1afc470817c56e31f0f9218a",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "3214c7409d46651be3b0819ae5923eed01d5950f1c5cd7224892e1e67790988e",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "62711c414c454aa159549f03310dad860fd0aacc54f8dc8f136e570dc46d7ae5",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "5bea94073ecb833cd9b6b8ec185d223c3ba4a0668a2794361329827406651770",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "4ad0785944f1bd2f35da51a1a2117b6f0067ba8f0543e8f1b470ce63f343d2f8",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "3aacd5c148f61eecabfe9e02f697b065acf38b317d6e579c641dbf0912c89be4",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "3aff0cb1d902853d34085b2ed39da88cdc99c04707316826858e3b34c3318301",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "2ccf9f732bd99e7f09ef33788fd327c668e49c13261f22839226430bf4c53505",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "110e7e86f4cd251d29252e48ad29d5f98ee311efedad56c987c5c7eb3a20405e",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "05c557e0b9693a3753172709a02e3bb712888628f0b0200c57c0d89ed603f5e4",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "289fef0841861205b6b93f4e634421fb61f85bf6fa0fd85cf776bbb089c20eb0",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "6e154c178fe9875ae15b7ea08bed25dd7a3f263011f9fdaed8f9ce311fc97e6f",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "6afbf642a95ca31913789765753a7fe79b16e4e78335c94fcf616336fed69abf",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "12c093cedb801ed9da2956b690ead4897d1d167b2b0cf5615de55070f913a8cc",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0db4b851b95038c47deb6ada0dad70e098fc3da4b00e7b9a7da8de0c62f5d2c1",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "33aa8799c6635ce6cee75572dac7d7fd06969da0a11ae310fc147f9308b8190f",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "212cd0c1c8d589a6e2262d5c87ca5ab66da2ba9b1a0a6d278348f588fc156cb1",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "233c6f29e8e21a866ec2bfe15060033c78f51a8967d33f1faf0ff51ebd085cf2",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "1db7778911914ce3a70a862a3d3d3341122ecdf2527e9d28d2f4d5107f18c781",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "7ae2ca8a017d24be15df4161fd2ac852e2b8ded419cf8da5b3394769dd701ab6",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "31ad97ad26e2adfc7c74f43abf79a3307a97e2cc53d50113ddf352397c6bc26f",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "1ed1fc53a6622c83e491c14f25560a641e8518cc3f19da9db7e817ed0920b962",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "503d664a57aa24ad3e19167bbf3712fb73504898c9bf388e8bfe42a61c092d2d",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "5b12b36f28bc0aa323db7e6d0485c45cb382df288570b7e7f4b9e98e4d89f26e",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "36906685e9a1f8e3192a023e0c8e8bfab9ba83f7cf37e5f2ca1b395b90a91537",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "355dccf04805c3a5eb27ee64be5b3d392fcf1dd2138b57e4544cbe3c4fd8781d",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "5b1112708474b19e56011dc0abd8215ba4700cfa31d75c7c6b190dd8b8699e48",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "584587b225ae4f65b138b588b65981961fb65ee757f6567fcbbd984dcb3c75db",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "05e27ba4b982ac548b4fffd380b2218b84eb616ccb9197c44855c10f66a67ed6",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "3d8918fb87d11eefdb481808a9805bb75435d15b33bc2bee3393a363f12f57a6",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "7f8f3424d64a55da6b61108eeb682acfe5aeadb0de2a10863f06a67d1e5a864d",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "7e755cba0310f265af5231a47a45654f9548ed1ec442fde27b1a4807b24886af",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0caa7059c32356c4aafdfe16ce23bcc3ee1bf0c768f0f68b0ed6293624794ed1",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4536c2aee70b3230e246f40cfde45959a64e1a882696f6689afc4f52761a3023",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "6a15d0f5ca4497b3dc605fed0f63168b9a38bf74652157b88ce3eff321ccb9c3",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "7bbdf8041ba47471fe1bf852fe71018d6eb4e737f02fc226e019a302599db7fa",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "55c206d4035cdb7afb2f399a36978858dc24eaf9e221c4f08e0de1f109bfa8d5",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "5cd6b3922ee71c250f99db5c214a59cb8227d19e1431c8e30a27faad90de7625",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "278fc8bcd74e9eb8599c14c00d1e9efc02b3db6ad6cf64f971538159b8443d37",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "468615291ab8842853f4309ee0cd142be111126e7a37a7bef03ca994d633ebc7",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "5523e2f353889485f4ad215318e2792bed2229eacdfd6b672c403851d54ceb6f",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "636db66a5894edd3dc18b38f13d4fbcff5c506a0f0579f9771a1099c54a5efd2",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "5288cf65559b0f9888f5a27cb3c84c350f6125ab65a3f1e29afa536e7bd0d4de",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "12c70c85f45241d25fee4f89484bbafbfcb1fb13e5570e710f92b629f0d9881c",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "63f01b555a9646146dd6f2c60f76892958feabbaea812a3fb295c8c50a97289b",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "437165416ab629553c939f69d4d5335160299307d30960cc6a45bda5e538767f",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "46fe985f1b9721b76b7819199980dd1fad81bfbae4b6778de1d5b1fbddfdad86",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "3ea46dc72c2dd23c8465234f47f364755036a4d0f4953a7161300a2836e64b9a",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "088b0ca7df43294e6a0e5e97d89c74e56739f401fd075eea9ff5018588e2dfa7",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "3c755700af5ee893fa9f0e23f535a3df0ebb9d53a8e51d9410670e54fefe6cc0",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "7d5b0546110e1379ed8fa1d695119b5876d08ebe7e436fcd264445337c54aa9d",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "1ae5c564b3a77aad64164668d421c0d7aaf8fe7b0afb01f3789f3a96d7c70596",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "201a641198d926634500e7c00885e2a413fbe6d47c6a5b84da09ad4c0302594b",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "5d9cf1e818af17860f307ce81e745d9049e5adc4c8a5b2a5fd88e6863e708d5b",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "3aa0a0c361fe0b265579493cdc424c4b2bdbaaaa62b8c41f5bda1d3be2a1592b",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "3e6190f708b203404726e9dd5c77a544c7d012ab660f838d941c5fe508dff693",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "6ee309f230d1a129b0aa755c9e78b3308ce0c10250067169067c6e21e149ef2e",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "64cde98364f1d74b617424f7b0e849ad7a22228f547ec2093948ae32ac67b877",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "19767cc1442030072d22795787953b0fb8b41b10c61f05c842feb982b66c4ffa",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2c41a80e5b45383184f2fda400294fb37d38f68fbe1f51f7270fd6e4071f6450",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "393bc7b77c81c3e8af0cb14fdaca4ec472eeef35978e184d05be0fe08e9dc54b",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "6ebb5599ac3d36967becb9f64905d2b4568513fa38cd6ec9b13b67a868cd8c15",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "7c0cebbd0ca4ee63f7b9153bc5c0c17a9f5cb50d883b9b0ae9bb8645b73f4755",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "7cead1176a994c8c306604d0cfac969b266debe5677e65fa429cc5da306059bd",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "7b816374fe4d0adcdd36b61c27b01208472b3b3d7b8c9150621b1e08c64de622",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "32e73d7c414d75517fcb93e1232ee3d313328741d66c12c336fe4cdb68564783",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "6f56ae3ce96f01638f279c75d65fa4145c3c7d0f88ca735852971bc104113fcc",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "2bcbc96fc92ab0e94573db8e664d36bf81665fa0f3257935477aa3e186f6b4b9",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "6ff27a9feafb3d179b8d37a2d4f2f726df936b434616d65d5d3896ed8c1e9273",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "72b5a5b6de2848cd50e00475a46e2c35f033b146a881ce416c09f73e611f6329",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "3485a7aa6fde7d086ba40318852439779a1c97a3633fa3073c712c4628a337c3",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "61aa1160d97b7167fc516b395d91401df8dd09994bef14abf1369774ed68e720",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "2c6ce0503ee8d142b373ecf168e0884c374bcb75d5862d9725348a7b7f55128e",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "0f8c2b53783393a70ee0a9b803128a43340cece1e021e31e894f17e676469b1a",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "0e77ad1d926497a9e4eee56c9af6921a5e96ccdc292a44f28d82dfb19c632889",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4cc1e54c7258ddf578e002eeb89dad4fdd40e090a80d19de2f1a301df2db5c75",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "7294f2237a32de778645c39ffe3413f9031ea7947980bd38b89be1d86b3ae19c",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "62ef3fdce75142e4f5c9aea740a9bb66f4261f78644e48fbf3fcc0667543638e",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "1928c87d156662a4bda9ce374f0265de9ed1dd9e3869f2436e588017f77d3efd",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "4ae0ec1d4499fa945755a86075ce291b2da9a2cab59758c64a3847d566087229",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "77db41774458f630420474ed08bd2c996feb7fbd9644f5f51b0c955ab57e2130",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "17f1b3461da3170eaebfa49793d0bfc4866b1d980ce07dd583d6cb9ba2be7da7",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "78a82c43f443d24dabc758e794417b00bac75d4ee3e4e79a09a16b3d036c2886",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "3d9b99a13ada9626d1762d7b4576a7352ca3611a7b6710e58e4c199b3403ce52",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "0b49208bd81bb3904539a75af7c4ac7fefd139eb4469474e056b8112702675c4",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "3e40a64da2d51448c16393986f009808eac8ed716e98eed603fd50fb0a0d0782",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "157ee7b2e1f2852187ac12fcf368d80e7b3d1775c49584c135badcb32d287241",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "5a277115c55fbeb41f5baef6f12e3f952522d09bc73ffcbd97f5a52e9dca709f",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "0f4e191892dd3d73e3636e504e122e6e273c7b386fde31b27b40d921e5854c55",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "4689b02ab17dfeee4f382d7d586db8f0c87ec77fd587a7c2ab123015328300cc",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "3b2432ebc9edd62786f493945f708794e1f46058d922b5bdc3bd1c12ec4132ed",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "361fd1330328d0c1b4222b623836c62fa42b5a086f398c281700899781c7d8ef",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "6c55c1f2ab2dbbb98dff3409edc0963d0dd9373831c17c95bbbeccc2b78c2e59",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "3cd845a927b2c486b1596816d4fa0444b897391177c50f44d22b0c8165159986",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "45f998ac7247f2ad556ab0a2481b8e75fbea3365e8f82ff512f506d72c1951df",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "36121e8a0da91ad10f566b56d20e46dc40722a7e78d75eb71d1715addf6fd3b0",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "23b086cf066d531bff46c2d572544cced6e1434c46695337a40b0728c55d3ecd",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "16c5fa19014f161544fe4535012cbc39bd7ca4098634ba318d666080b4bdd58f",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "01ebb5388c6e8a74ea3385092cd28f32b1d4514237c16905c36b5118ea05195e",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "4864ef1818473050affd29c356960710c98ff8c5cb9ce8950f45416ee772f53b",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "26c03aed7f6bc250e66f3e9b04153ef4864eeb9bb91c3428cd0da83a0bd0b830",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "2a7adc0c34dbaf6c5cc04b883b623c0f3447c7ab2068d38ef817c33297639ab3",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "78b5169959e1d75490d94a7bb989a43c3a9e89604829a96d961e7a2f581c7dce",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "457ec0224bcd21b41dcb8a165586a836e6fe953a574dd32bf95cc85a5769cc40",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "02314bc90371e70927572a35488e711f75061507fce9d46510f98254df5d180d",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "79572c534fcf0a49bb8b7936ea84cefa16bbb31dfb553ad379e8aefe8f26908a",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "7890c0b6e7f19a1c1493f6ab9688c9a45f372623906404c4343b4300e0749597",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "70ddf8d98b60ef3f043337880d96ea04bb0a0dbab92cc1b9239db23ca35b2d6f",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "45769691e89a70aa2b58e231ecb48485a3b303985af8169bf49140b7fdd75dc4",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "2ec0f706b05c7c788bae741056679d90885cfed9e9b91b6b390e3ddc5ba643ad",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "10b74232f01c1d8202235defdd55e17d47a9cc35ec02eb9b54e3e305345b2ddb",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "5d6fa9d25a3f46a79050b66e2a4e1470894dee1fbe3d0c7cfc3a1694608f59d8",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "574fa41887c9e71c064ccbf6189f9cada66ced079df00e65ac2c5afeb2a3a6dd",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "645e704f775f697a797ff5f7f57d26462a69cc012f223e5246df4185e46e6cbb",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "7420e574dcaab932f2355982294e15110f3a73ed122ee7d8c92d29dade891efa",
            )),
        },
        AffineNielsPoint {
            y_plus_x: FieldElementR0(U256::from_be_hex(
                "2ba60fa9c3cdc075454b33b093c454a5c87b61e69b421c10f83e6e3f94234b1c",
            )),
            y_minus_x: FieldElementR0(U256::from_be_hex(
                "44ef4632b581b3d328ae39018d46e4a60fe6202f46f9d39df766a138a034513c",
            )),
            xy2d: FieldElementR0(U256::from_be_hex(
                "7054899c44b5f3cf56f511d9c0071c9745da3bac65224a25a3d6491c21d364c9",
            )),
        },
    ]);
