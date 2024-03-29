RUSTFLAGS = "-C link-arg=-s"
PACKAGE_NAME = "hello_test"

lint-contract:
	@cargo fmt --all
	@cargo clippy --fix --allow-dirty --allow-staged

lint-integration:
	@yarn
	@yarn lint
	@yarn prettier

lint: lint-contract lint-integration

build:
	@rustup target add wasm32-unknown-unknown
	RUSTFLAGS=$(RUSTFLAGS) cargo build --target wasm32-unknown-unknown --release
	@mkdir -p res
	@cp target/wasm32-unknown-unknown/release/$(PACKAGE_NAME).wasm ./res/

test-unit:
	@cargo test

test-integration:
	@yarn jest --testTimeout 60000 --detectOpenHandles

test: test-unit test-integration

all: lint build test
