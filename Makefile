rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cd lyric && cargo fmt 

lint:
	cd lyric && cargo clippy 

test:
	cd lyric && cargo test --quiet

run:
	cd lyric && cargo run

release:
	cd lyric && cargo build --release

all: format lint test run