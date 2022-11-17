all: debug
  
.PHONY: debug
debug:
	RUST_BACKTRACE=full cargo build
#	cargo build

.PHONY: release
release:
	cargo build --release

.PHONY: clean
clean:
	rm -rf target
	rm Cargo.lock
	rm ../libs/protocols/src/image*
