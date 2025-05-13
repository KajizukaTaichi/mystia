import { mystia as compile } from "../wasm/node/mystia_wasm.js";
import { read, write } from "./ffi.mjs";

export async function mystia(code) {
    const result = compile(code);
    const type = eval(`(${result.get_return_type()})`);
    const bytecodes = result.get_bytecode().buffer;

    let mystiaFunctions = {
        print: null,
        int_to_str: null,
        concat: null,
        rand: null,
    };
    const { instance } = await WebAssembly.instantiate(bytecodes, {
        env: {
            print: (ptr) => mystiaFunctions.print(ptr),
            int_to_str: (num) => mystiaFunctions.int_to_str(num),
            concat: (str1, str2) => mystiaFunctions.concat(str1, str2),
            rand: () => mystiaFunctions.rand(),
        },
    });
    mystiaFunctions.print = (ptr) => {
        return console.log(read(instance, "str", ptr));
    };
    mystiaFunctions.int_to_str = (value) => {
        return write(instance, "str", value.toString());
    };
    mystiaFunctions.concat = (str1, str2) => {
        str1 = read(instance, "str", str1);
        str2 = read(instance, "str", str2);
        return write(instance, "str", str1 + str2);
    };
    mystiaFunctions.rand = () => {
        return Math.random();
    };
    const value = instance.exports._start();
    return read(instance, type, value);
}
