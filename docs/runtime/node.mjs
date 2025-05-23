import { mystia as compile } from "../wasm/node/mystia_wasm.js";
import { MystiaNodeLib } from "./lib.mjs";
import { MathLib } from "./math.mjs";
import { OSLib } from "./os.mjs";
import { read } from "./ffi.mjs";

export async function mystia(code) {
    const result = compile(code);
    const type = eval(`(${result.get_return_type()})`);
    const bytecodes = result.get_bytecode().buffer;

    let mystiaStdLib = new MystiaNodeLib();
    let math = new MathLib();
    let os = new OSLib();
    const { instance } = await WebAssembly.instantiate(bytecodes, {
        env: { ...mystiaStdLib.bridge(), ...math.bridge(), ...os.bridge() },
    });
    mystiaStdLib.set_wasm(instance);
    math.set_wasm(instance);
    os.set_wasm(instance);

    const value = instance.exports._start();
    return read(instance, type, value);
}
