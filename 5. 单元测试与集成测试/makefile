RUSTFLAGS = "-C link-arg=-s"
PACKAGE_NAME = "hello_test"

lint-contract:
	@cargo fmt --all
	@cargo clippy --fix --allow-dirty --allow-staged

lint-integration:
	@yarn
	@yarn eslint tests/**/*.ts --fix
	@yarn prettier tests/**/*.ts --write

lint: lint-contract lint-integration

compile:
	@rustup target add wasm32-unknown-unknown
	RUSTFLAGS=$(RUSTFLAGS) cargo build --target wasm32-unknown-unknown --release
	@mkdir -p res
	@cp target/wasm32-unknown-unknown/release/$(PACKAGE_NAME).wasm ./res/

build: lint-contract compile

test-unit:
	@cargo test

test-integration:
	@yarn test

test: lint compile test-unit test-integration
