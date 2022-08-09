default:
	cargo clean
	cargo fmt
	cargo clippy
	cargo build

test:
	cargo clean
	cargo fmt
	cargo clippy
	cargo test


release:
	cargo fmt
	cargo clippy
	cargo test
	cargo clean
	cargo build --release
