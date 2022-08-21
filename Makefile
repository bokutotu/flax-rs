default:
	cargo clean
	cargo fmt
	cargo clippy
	cargo test
	cargo build

test_cargo:
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

commit: 
	cargo clean
	cargo fmt
	cargo clippy
	cargo test
	git add .
	git commit
