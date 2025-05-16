import { mystia as compile } from "../wasm/node/mystia_wasm.js";
import { read } from "./ffi.mjs";
import { MystiaStdLib } from "./lib.mjs";

export async function mystia(code) {
    const result = compile(code);
    const type = eval(`(${result.get_return_type()})`);
    const bytecodes = result.get_bytecode().buffer;

    let mystiaStdLib = new MystiaStdLib();
    const { instance } = await WebAssembly.instantiate(bytecodes, {
        env: mystiaStdLib.bridge(),
    });
    mystiaStdLib.set_wasm(instance);
    const value = instance.exports._start();
    return read(instance, type, value);
}
