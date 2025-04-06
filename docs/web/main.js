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
    if (type == "int" || type == "num") {
        return value;
    } else if (type == "bool") {
        return value != 0;
    } else if (type == "str") {
        const memoryView = new Uint8Array(instance.exports.mem.buffer);
        let stringLength = value;
        while (memoryView[stringLength] !== 0) {
            stringLength++;
        }
        const stringBytes = memoryView.slice(value, stringLength);
        const textDecoder = new TextDecoder("utf-8");
        return textDecoder.decode(stringBytes);
    } else if (type.startsWith("[") && type.endsWith("]")) {
        let result = [];
        const [innerType, length] = rsplitOnce(";");
        const bytes = innerType == "num" ? Uint16Array : Uint8Array;
        const memoryView = new bytes(instance.exports.mem.buffer);
        for (let index = 0; index < length; index++) {
            result.push(ffi(innerType), memoryView[index]);
        }
        return result;
    }
}

function rsplitOnce(str, delimiter) {
    const idx = str.lastIndexOf(delimiter);
    return [str.slice(0, idx), str.slice(idx + delimiter.length)];
}
