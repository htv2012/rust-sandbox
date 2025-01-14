.PHONY: \
	all \
	build \
	format \
	lint \
	test \


all: test

test: build lint format
	cargo test

lint:
	cargo clippy

format:
	cargo fmt

build:
	cargo build


