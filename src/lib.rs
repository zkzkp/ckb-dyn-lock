#![no_std]

#[cfg(not(feature = "lock_binary"))]
extern crate alloc;

#[cfg(any(feature = "lock_binary", feature = "test_tool"))]
extern crate std;

pub mod locks;
#[cfg(feature = "test_tool")]
pub mod test_tool;

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
