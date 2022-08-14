_default:
	just --list

# Open crate documentation.
docs:
	cargo docs --open --no-deps

# Run tests matching `TEST`. Leave blank to run all tests.
test TEST='':
	cargo test {{TEST}}

# Build in release mode.
build:
	cargo build --release

# Run in debug mode.
debug ARGS='':
	cargo run {{ARGS}}

# Run in release mode.
run-release ARGS='': build
	./target/release/rover {{ARGS}}


