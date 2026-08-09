## this will the makefile for this application

.PHONY: run build release test fmt lint clean

run:
	cargo run

build:
	cargo build

release:
	cargo build --release

test:
	cargo test

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

clean:
	cargo clean
