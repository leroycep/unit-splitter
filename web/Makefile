default:
	cargo build --target wasm32-unknown-unknown --release
	mkdir -p target/unit-splitter
	cp static/* target/unit-splitter -rf
	wasm-bindgen target/wasm32-unknown-unknown/release/unit_splitter_web.wasm --out-dir target/unit-splitter --no-modules --no-modules-global UnitSplitter
	wasm-gc target/unit-splitter/unit_splitter_web_bg.wasm

serve: default
	miniserve target/unit-splitter
