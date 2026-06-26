build-dashu:
	@cargo build -r --no-default-features --features cli,dashu

# Could only compile with dashu as backend for wasm.
build-wasm:
	@wasm-pack build --scope breezewhite_yo --target web --no-default-features --features wasm,dashu
	@cp README_WASM.md pkg/README.md

publish-wasm:
	@wasm-pack publish --access=public