.PHONY: package build aws

SHELL := /usr/bin/env bash

build:
	cargo build --release --target x86_64-unknown-linux-musl

package: build
	cp ./target/x86_64-unknown-linux-musl/release/bootstrap ./docker	
	cd docker && docker build -t rust_lambda .

aws: package
	cd infra && source .venv/bin/activate && cdk deploy