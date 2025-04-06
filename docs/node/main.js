import { mystia as compile } from "./mystia_wasm.js";

export async function mystia(code) {
    const result = compile(code);
    const type = result.get_return_type();
    const bytecodes = result.get_bytecode().buffer;
    const { instance } = await WebAssembly.instantiate(bytecodes);
    const value = instance.exports._start();
    return ffi(instance, type, value);
}

function ffi(instance, type, value) {
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
        type = type.slice(1, type.length);
        let [innerType, length] = rsplitOnce(type, ";");
        length = parseInt(length.trim());
        const [arrayClass, byte] =
            innerType == "num" ? [BigUint64Array, 8] : [Uint32Array, 4];
        const memoryView = new arrayClass(instance.exports.mem.buffer);
        let result = [];
        let index = value / byte;
        while (index < index + length) {
            result.push(ffi(instance, innerType, memoryView[index]));
            index++;
        }
        return result;
    }
}

function rsplitOnce(str, delimiter) {
    const idx = str.lastIndexOf(delimiter);
    return [str.slice(0, idx), str.slice(idx + delimiter.length)];
}
