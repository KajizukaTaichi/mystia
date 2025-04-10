export function ffi(instance, type, value) {
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
        const token = type.slice(1, type.length);
        const [innerType, lengthSource] = rsplitOnce(token, ";");
        const length = parseInt(lengthSource.trim());
        const [arrayClass, byte] =
            innerType == "num" ? [BigInt64Array, 8] : [Int32Array, 4];
        const memoryView = new arrayClass(instance.exports.mem.buffer);
        const pointer = value / byte;
        let [result, index] = [[], pointer];
        while (index < pointer + length) {
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
