#![no_std]

#[cfg(not(feature = "lock_binaries"))]
extern crate alloc;

#[cfg(feature = "lock_binaries")]
extern crate std;

pub mod locks {
    //! pub use const CODE_HASH_SECP256K1_KECCAK256_SIGHASH_ALL_DUAL: [u8; 32]
    //! pub use const CODE_HASH_SECP256K1_DATA: [U8; 32]

    include!(concat!(env!("OUT_DIR"), "/code_hashes.rs"));

    #[cfg(feature = "lock_binaries")]
    pub mod binaries {
        //! pub use const BUNDLED_CELL: Files
        include!(concat!(env!("OUT_DIR"), "/bundled.rs"));
    }
}

use ckb_std::dynamic_loading::{CKBDLContext, Symbol};

type VerifySecp256k1KeccakSighashAll = unsafe extern "C" fn(eth_address: *const [u8; 20]) -> i32;

const VERIFY_SECP256K1_KECCAK_SIGHASH_ALL: &[u8; 35] = b"verify_secp256k1_keccak_sighash_all";

pub struct DynLock {
    verify_signhash: Symbol<VerifySecp256k1KeccakSighashAll>,
}

impl DynLock {
    pub fn load<T>(context: &mut CKBDLContext<T>, code_hash: &[u8]) -> Self {
        let lock = context.load(code_hash).expect("load lock lib");

        let verify_signhash: Symbol<VerifySecp256k1KeccakSighashAll> = unsafe {
            lock.get(VERIFY_SECP256K1_KECCAK_SIGHASH_ALL)
                .expect("load sign hash fn")
        };

        DynLock { verify_signhash }
    }

    pub fn verify(&self, hash: &[u8; 20]) -> Result<(), i32> {
        let verify_signhash = &self.verify_signhash;
        let error_code = unsafe { verify_signhash(hash) };

        if error_code != 0 {
            return Err(error_code);
        }
        Ok(())
    }
}
