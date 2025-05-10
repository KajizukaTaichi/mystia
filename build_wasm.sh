cd ./wasm
wasm-pack build --target nodejs
mv pkg/* ../docs/wasm/node/
wasm-pack build --target web
mv pkg/* ../docs/wasm/web/
