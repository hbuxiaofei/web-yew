build:
	wasm-pack build --target web --out-dir ./static/wasm
	./node_modules/.bin/rollup ./static/index.js --format iife --file ./static/wasm/index.out.js


prepare:
	cargo install wasm-pack
	npm install rollup


run:
	cd static && python3 -m http.server 8080
