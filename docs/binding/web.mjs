import init, { mystia as compile } from "../web/mystia_wasm.js";
import { ffi } from "./ffi.mjs";

await init();
export async function mystia(code) {
    const result = compile(code);
    const type = result.get_return_type();
    const bytecodes = result.get_bytecode().buffer;
    const { instance } = await WebAssembly.instantiate(bytecodes);
    const value = instance.exports._start();
    return ffi(instance, type, value);
}
