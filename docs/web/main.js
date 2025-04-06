import init, { mystia as compile } from "./mystia_wasm.js";

await init();
export async function mystia(code) {
    const result = compile(code);
    const type = result.get_return_type();
    const bytecodes = result.get_bytecode().buffer;
    const { instance } = await WebAssembly.instantiate(bytecodes);
    const value = instance.exports._start();
    return ffi(type, value);
}

function ffi(type, value) {
    if (type == "str") {
        let stringLength = value;
        const memoryView = new Uint8Array(instance.exports.mem.buffer);
        while (memoryView[stringLength] !== 0) {
            stringLength++;
        }
        const stringBytes = memoryView.slice(value, stringLength);
        const textDecoder = new TextDecoder("utf-8");
        return textDecoder.decode(stringBytes);
    } else if (type == "int" || type == "num") {
        return value;
    } else if (type == "bool") {
        return value != 0;
    }
}
