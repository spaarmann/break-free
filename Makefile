.PHONY: web
web:
	cargo build --target wasm32-unknown-unknown
	wasm-bindgen --no-typescript --target web \
		--out-dir ./out/ \
		--out-name "break-free" \
		./target/wasm32-unknown-unknown/debug/break-free.wasm
	cp ./web/* ./out/
