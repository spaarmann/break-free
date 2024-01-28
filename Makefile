.PHONY: web
web:
	cargo build --target wasm32-unknown-unknown
	wasm-bindgen --no-typescript --target web \
		--out-dir ./out/ \
		--out-name "break-free" \
		./target/wasm32-unknown-unknown/debug/break-free.wasm
	cp ./web/* ./out/

web-release:
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --no-typescript --target web \
		--out-dir ./out/ \
		--out-name "break-free" \
		./target/wasm32-unknown-unknown/release/break-free.wasm
	cp ./web/* ./out/
	wasm-opt -Oz -o ./out/break-free_bg.wasm ./out/break-free_bg.wasm
