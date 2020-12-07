


build:
	wasm-pack build --target web
	npm i rollup
	./node_modules/.bin/rollup ./index.js --format iife --file ./pkg/bundle.js


run:
	python3 -m http.server 8080
