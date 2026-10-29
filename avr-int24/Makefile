.PHONY: test clean doc

test:
	cargo fmt --all --check
	cargo clippy -- --deny warnings
	cargo test

clean:
	cargo clean

doc:
	cargo doc --open
