.PHONY: package build

build:
	cargo build --release --target x86_64-unknown-linux-musl

package: build
	cp ./target/x86_64-unknown-linux-musl/release/bootstrap ./docker	
	cd docker && docker build -t rust_lambda .