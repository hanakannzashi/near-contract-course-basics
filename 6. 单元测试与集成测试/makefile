RUSTFLAGS = "-C link-arg=-s"
PACKAGE_NAME = "hello_test"

lint-contract:
	@cargo fmt --all
	@cargo clippy --fix --allow-dirty --allow-staged

lint-integration:
	@pnpm install
	@pnpm lint
	@pnpm prettier

lint: lint-contract lint-integration

build:
	@rustup target add wasm32-unknown-unknown
	RUSTFLAGS=$(RUSTFLAGS) cargo build --target wasm32-unknown-unknown --release
	@mkdir -p res
	@cp target/wasm32-unknown-unknown/release/$(PACKAGE_NAME).wasm ./res/

test-unit:
	@cargo test

test-integration:
	@pnpm test

test: test-unit test-integration

all: lint build test
