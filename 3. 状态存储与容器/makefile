RUSTFLAGS = "-C link-arg=-s"
PACKAGE_NAME = "hello_collections"

lint:
	@cargo fmt --all
	@cargo clippy --fix --allow-dirty --allow-staged

compile: .
	@rustup target add wasm32-unknown-unknown
	RUSTFLAGS=$(RUSTFLAGS) cargo build --target wasm32-unknown-unknown --release
	@mkdir -p res
	@cp target/wasm32-unknown-unknown/release/$(PACKAGE_NAME).wasm ./res/

build: lint compile
