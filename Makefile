pw-lock:
	cd deps/pw-lock && make all-via-docker
	cp deps/pw-lock/specs/cells/secp256k1_keccak256_sighash_all_dual specs/cells/
	cp deps/pw-lock/specs/cells/secp256k1_data specs/cells/
	cp deps/pw-lock/specs/cells/secp256k1_keccak256_sighash_all specs/cells/

all: pw-lock

.PHONY: all pw-lock
