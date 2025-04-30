export function ffi(instance, type, value) {
    if (type == null) return null;
    if (type == "int" || type == "num") {
        return value;
    } else if (type == "bool") {
        return value != 0;
    } else if (type == "str") {
        const memoryView = new Uint8Array(instance.exports.mem.buffer);
        let stringLength = value;
        while (memoryView[stringLength] != 0) {
            stringLength++;
        }
        const stringBytes = memoryView.slice(value, stringLength);
        const textDecoder = new TextDecoder("utf-8");
        return textDecoder.decode(stringBytes);
    } else if (type.type == "array") {
        const [innerType, length] = [type.element, type.length];
        const [arrayClass, byte] =
            innerType == "num" ? [Float64Array, 8] : [Int32Array, 4];
        const memoryView = new arrayClass(instance.exports.mem.buffer);
        const pointer = value / byte;
        let [result, index] = [[], pointer];
        while (index < pointer + length) {
            result.push(ffi(instance, innerType, memoryView[index]));
            index++;
        }
        return result;
    } else if (type.type == "dict") {
        const [pointer, result] = [value, {}];
        const memoryView = new Uint8Array(instance.exports.mem.buffer);
        for (let [name, field] of Object.entries(type.fields)) {
            const address = pointer + field.offset;
            const value = (() => {
                int32PairToFloat64;
                if (field.type == "num") {
                    const sliced = memoryView.slice(address, address + 8);
                    return int32PairToFloat64(sliced, true);
                } else {
                    const sliced = memoryView.slice(address, address + 4);
                    return int32PairToFloat64(sliced);
                }
            })();
            result[name] = ffi(instance, field.type, value);
        }
        return result;
    } else if (type.type == "enum") {
        return type.enum[value];
    } else {
        return type;
    }
}

function int32PairToFloat64(bytes, is_64bit = false) {
    const buffer = new ArrayBuffer(8);
    const view = new DataView(buffer);
    let index = 0;
    for (let byte of bytes) {
        view.setUint8(index, byte);
        index += 1;
    }
    return is_64bit ? view.getFloat64(0, true) : view.getInt32(0, true);
}
