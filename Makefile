format:
	cargo fmt

lint:
	cargo clippy

test:
	cargo test

run:
	cargo run -- --file test.txt
