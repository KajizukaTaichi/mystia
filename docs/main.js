import init, { mystia as compile } from "./pkg/mystia_wasm.js";

export async function mystia(code) {
    await init();
    const { instance } = await WebAssembly.instantiate(compile(code).buffer);
    return instance.exports._start();
}
