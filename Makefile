.PHONY: cargo_test cargo_build docker_build lambda_local_run cdk_deploy cdk_destroy

SHELL := /usr/bin/env bash

cargo_test:
	cargo test

cargo_build:
	cargo build --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/bootstrap ./docker

docker_build: cargo_build
	cd docker && docker build -t rust_lambda .

lambda_local_run: cargo_build
	docker run --rm -e DOCKER_LAMBDA_STAY_OPEN=1 -p 9001:9001 -v $(PWD)/docker/bootstrap:/var/task/bootstrap:ro,delegated lambci/lambda:provided main

cdk_deploy: cargo_build
	cd infra && source .venv/bin/activate && cdk deploy

cdk_destroy:
	-cd infra && source .venv/bin/activate && cdk destroy