pw-lock:
	cd deps/pw-lock && make all-via-docker
	cp deps/pw-lock/specs/cells/* specs/cells/

.PHONY: pw-lock
