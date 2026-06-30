build-dashu:
	@cargo build -r --no-default-features --features cli,dashu

# Could only compile with dashu as backend for wasm.
build-wasm:
	@wasm-pack build \
		--scope breezewhite_yo \
		--target web \
		--no-default-features \
		--features wasm,dashu
	@cp README_WASM.md pkg/README.md
	@cp -r pkg gh_page/

publish-wasm:
	@wasm-pack publish --access=public

check:
	@cargo check
	@cargo check --no-default-features --features cli,dashu
	@cargo check --no-default-features --features cli,wasm,dashu

test:
	@cargo test
	@cargo test --no-default-features --features cli,dashu