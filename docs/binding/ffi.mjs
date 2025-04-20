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
        const [pointer, result] = [value / 4, {}];
        const memoryView = new Int32Array(instance.exports.mem.buffer);
        for (let [name, field] of Object.entries(type.fields)) {
            const address = pointer + field.offset == 0 ? 0 : field.offset / 4;
            let value = (() => {
                if (field.type == "num") {
                    return int32PairToFloat64(
                        memoryView[address],
                        memoryView[address + 1],
                    );
                } else {
                    return memoryView[address];
                }
            })();
            result[name] = ffi(instance, field.type, value);
        }
        return result;
    }
}

function int32PairToFloat64(a, b) {
    const buffer = new ArrayBuffer(8);
    const view = new DataView(buffer);
    view.setInt32(0, a, true);
    view.setInt32(4, b, true);
    return view.getFloat64(0, true);
}
