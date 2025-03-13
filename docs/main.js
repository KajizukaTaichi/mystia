import init, { mystia as compile } from "./pkg/mystia_wasm.js";

await init();
export async function mystia(code) {
    let bytecodes = compile(code).buffer;
    const { instance } = await WebAssembly.instantiate(bytecodes);
    return instance.exports._start;
}
