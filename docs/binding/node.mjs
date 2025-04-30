import { mystia as compile } from "../node/mystia_wasm.js";
import { ffi } from "./ffi.mjs";

export async function mystia(code) {
    const result = compile(code);
    const type = eval(`(${result.get_return_type()})`);
    const bytecodes = result.get_bytecode().buffer;

    let mystiaFunctions = {
        print: null,
    };
    const { instance } = await WebAssembly.instantiate(bytecodes, {
        env: {
            print: (ptr) => mystiaFunctions.print(ptr),
        },
    });
    mystiaFunctions.print = (ptr) => console.log(ffi(instance, "str", ptr));

    const value = instance.exports._start();
    return ffi(instance, type, value);
}
