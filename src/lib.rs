#![no_std]

#[cfg(not(feature = "include_locks"))]
extern crate alloc;

#[cfg(feature = "include_locks")]
extern crate std;

#[cfg(feature = "include_locks")]
pub mod locks {
    //! pub use const BUNDLED_CELL: Files
    //! pub use const CODE_HASH_SECP256K1_KECCAK256_SIGHASH_ALL: [u8; 32]
    //! pub use const CODE_HASH_SECP256K1_KECCAK256_SIGHASH_ALL_ACPL: [u8; 32]

    include!(concat!(env!("OUT_DIR"), "/bundled.rs"));
    include!(concat!(env!("OUT_DIR"), "/code_hashes.rs"));
}

use ckb_std::dynamic_loading::{CKBDLContext, Symbol};

type LockMain = unsafe extern "C" fn() -> i32;

const LOCK_MAIN: &[u8; 4] = b"main";

pub struct DynLock {
    lock_main: Symbol<LockMain>,
}

impl DynLock {
    pub fn load<T>(context: &mut CKBDLContext<T>, code_hash: &[u8]) -> Self {
        let lock = context.load(code_hash).expect("load lock");
        let lock_main: Symbol<LockMain> = unsafe { lock.get(LOCK_MAIN).expect("load lock main") };

        DynLock { lock_main }
    }

    pub fn verify(&self) -> Result<(), i32> {
        let lock_main = &self.lock_main;
        let error_code = unsafe { lock_main() };

        if error_code != 0 {
            return Err(error_code);
        }
        Ok(())
    }
}
