pub fn main() {
    #[cfg(feature = "include_locks")]
    locks::package_locks()
}

#[cfg(feature = "include_locks")]
mod locks {
    use std::env;
    use std::fs::File;
    use std::io::{BufWriter, Read, Write};
    use std::path::Path;

    use blake2b_rs::{Blake2b, Blake2bBuilder};
    use includedir_codegen::Compression;

    const PATH_PREFIX: &str = "specs/cells/";
    const BUF_SIZE: usize = 8 * 1024;
    const CKB_HASH_PERSONALIZATION: &[u8] = b"ckb-default-hash";

    const BINARIES: &[(&str, &str)] = &[
        (
            "secp256k1_keccak256_sighash_all",
            "f3d9b5e8eff7bce00731be767a65f58cc5707c2b6ee8e732d56f383a47f77abd",
        ),
        (
            "secp256k1_keccak256_sighash_all_acpl",
            "f7c7fedcb0cbf549095f5b1ad6c267a141e96282b0f5308cad2d0af2901cfc48",
        ),
        (
            "secp256k1_data",
            "9799bee251b975b82c45a02154ce28cec89c5853ecc14d12b7b8cccfc19e0af4",
        ),
    ];

    pub fn package_locks() {
        let mut bundled = includedir_codegen::start("BUNDLED_CELL");

        let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("code_hashes.rs");
        let mut out_file = BufWriter::new(File::create(&out_path).expect("create code_hashes.rs"));

        let mut errors = Vec::new();

        for (name, expected_hash) in BINARIES {
            let path = format!("{}{}", PATH_PREFIX, name);

            let mut buf = [0u8; BUF_SIZE];
            bundled
                .add_file(&path, Compression::Gzip)
                .expect("add files to resource bundle");

            // build hash
            let mut blake2b = new_blake2b();
            let mut fd = File::open(&path).expect("open file");
            loop {
                let read_bytes = fd.read(&mut buf).expect("read file");
                if read_bytes > 0 {
                    blake2b.update(&buf[..read_bytes]);
                } else {
                    break;
                }
            }

            let mut hash = [0u8; 32];
            blake2b.finalize(&mut hash);

            let actual_hash = faster_hex::hex_string(&hash).unwrap();
            if expected_hash != &actual_hash {
                errors.push((name, expected_hash, actual_hash));
                continue;
            }

            write!(
                &mut out_file,
                "pub const {}: [u8; 32] = {:?};\n",
                format!("CODE_HASH_{}", name.to_uppercase()),
                hash
            )
            .expect("write to code_hashes.rs");
        }

        if !errors.is_empty() {
            for (name, expected, actual) in errors.into_iter() {
                eprintln!("{}: expect {}, actual {}", name, expected, actual);
            }
            panic!("not all hashes are right");
        }

        bundled.build("bundled.rs").expect("build resource bundle");
    }

    fn new_blake2b() -> Blake2b {
        Blake2bBuilder::new(32)
            .personal(CKB_HASH_PERSONALIZATION)
            .build()
    }
}
