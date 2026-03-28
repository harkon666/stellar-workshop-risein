.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/certificates.wasm ./

.PHONY: test
test:
	cargo test

.PHONY: clean
clean:
	cargo clean
	rm -f certificates.wasm