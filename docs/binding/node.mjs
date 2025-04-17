import { mystia as compile } from "../node/mystia_wasm.js";
import { ffi } from "./ffi.mjs";

export async function mystia(code) {
    const result = compile(code);
    const type = eval(`(${result.get_return_type()})`);
    if (type == null) return null;
    const bytecodes = result.get_bytecode().buffer;
    const { instance } = await WebAssembly.instantiate(bytecodes);
    const value = instance.exports._start();
    return ffi(instance, type, value);
}
