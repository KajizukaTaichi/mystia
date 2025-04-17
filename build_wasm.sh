cd ./wasm
wasm-pack build --target nodejs
mv pkg ../docs/node/
wasm-pack build --target web
mv pkg ../docs/web/
