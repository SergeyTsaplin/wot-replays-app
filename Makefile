setup:
	yarn install

build-wasm:
	cd app && cargo web build --release

copy-app:
	cp app/target/wasm32-unknown-unknown/release/todomvc.js src/
	cp app/target/wasm32-unknown-unknown/release/todomvc.wasm src/
	cp app/static/index.html src/

run: build-wasm copy-app
	yarn start

bundle: build-wasm copy-app
	yarn make