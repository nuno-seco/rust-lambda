.PHONY: package build aws

SHELL := /usr/bin/env bash

cargo_build:
	cargo build --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/bootstrap ./docker

docker_build: build
	cd docker && docker build -t rust_lambda .

cdk_deploy: build
	cd infra && source .venv/bin/activate && cdk deploy

cdk_destroy:
	-cd infra && source .venv/bin/activate && cdk destroy