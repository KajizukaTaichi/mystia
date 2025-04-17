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
        let result = {};
        const memoryView32bit = new Int32Array(instance.exports.mem.buffer);
        const memoryView64bit = new Float64Array(instance.exports.mem.buffer);
        for (let [name, field] of Object.entries(type.fields)) {
            const address = value + field.offset;
            const [memoryView, byte] =
                field.type == "num"
                    ? [memoryView64bit, 8]
                    : [memoryView32bit, 4];
            result[name] = ffi(
                instance,
                field.type,
                memoryView[address / byte],
            );
        }
        return result;
    }
}
