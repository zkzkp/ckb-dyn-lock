//! pub use const CODE_HASH_SECP256K1_DATA: [U8; 32]
//! pub use const CODE_HASH_SECP256K1_KECCAK256_SIGHASH_ALL: [u8; 32]
//! pub use const CODE_HASH_SECP256K1_KECCAK256_SIGHASH_ALL_DUAL: [u8; 32]

include!(concat!(env!("OUT_DIR"), "/code_hashes.rs"));

#[cfg(feature = "lock_binary")]
pub mod binary {
    //! pub use const BUNDLED_CELL: Files

    use std::borrow::Cow;

    include!(concat!(env!("OUT_DIR"), "/bundled.rs"));

    const BINARIES: [&str; 3] = [
        "specs/cells/secp256k1_data",
        "specs/cells/secp256k1_keccak256_sighash_all",
        "specs/cells/secp256k1_keccak256_sighash_all_dual",
    ];

    #[repr(u8)]
    pub enum Binary {
        Secp256k1Data = 1,
        Secp256k1Keccak256SighashAll,
        Secp256k1Keccak256SighashAllDual,
    }

    pub fn get(binary: Binary) -> Cow<'static, [u8]> {
        inner_get(binary).expect("should be packaged")
    }

    fn inner_get(binary: Binary) -> std::io::Result<Cow<'static, [u8]>> {
        BUNDLED_CELL.get(BINARIES[binary as usize - 1])
    }

    #[cfg(test)]
    mod tests {
        use super::{inner_get, Binary};

        #[test]
        fn test_get() {
            assert!(inner_get(Binary::Secp256k1Data).is_ok());
            assert!(inner_get(Binary::Secp256k1Keccak256SighashAll).is_ok());
            assert!(inner_get(Binary::Secp256k1Keccak256SighashAllDual).is_ok());
        }
    }
}
