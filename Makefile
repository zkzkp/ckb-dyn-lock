pw-lock:
	cd deps/pw-lock && make all-via-docker
	cp deps/pw-lock/specs/cells/secp256k1_keccak256_sighash_all_dual specs/cells/
	cp deps/pw-lock/specs/cells/secp256k1_data specs/cells/

.PHONY: pw-lock
