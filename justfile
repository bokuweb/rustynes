name = "rustnes"
exported_functions = "'_main'"

build:
	mkdir -p wasm
	rm -rf target/wasm32-unknown-emscripten/release/deps/*.wasm
	rm -rf target/wasm32-unknown-emscripten/release/{{name}}.js
	cargo rustc --release --verbose \
	--target=wasm32-unknown-emscripten -- \
	-C link-args="-s ASYNCIFY=1 -s NO_EXIT_RUNTIME=1 -s EXPORTED_FUNCTIONS=[{{exported_functions}}]" \
	--verbose 
	cp target/wasm32-unknown-emscripten/release/{{name}}.js wasm/{{name}}.js
	cp target/wasm32-unknown-emscripten/release/deps/*.wasm wasm/{{name}}.wasm