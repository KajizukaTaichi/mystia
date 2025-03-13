import init, { mystia as compile } from "./pkg/mystia_wasm.js";

await init();
export async function mystia(code) {
    const { instance } = await WebAssembly.instantiate(compile(code).buffer);
    return instance.exports._start;
}
