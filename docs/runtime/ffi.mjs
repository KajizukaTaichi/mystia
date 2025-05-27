export function read(instance, type, value) {
    if (type == null) return undefined;
    if (type == "int" || type == "num") {
        return value;
    } else if (type == "bool") {
        return value != 0;
    } else if (type == "str") {
        if (value == -1) return null;
        const memoryView = new Uint8Array(instance.exports.mem.buffer);
        let stringLength = value;
        while (memoryView[stringLength] != 0) {
            stringLength++;
        }
        const stringBytes = memoryView.slice(value, stringLength);
        const textDecoder = new TextDecoder("utf-8");
        return textDecoder.decode(stringBytes);
    } else if (type.type == "array") {
        if (value == -1) return null;
        const [innerType, length] = [type.element, type.length];
        const memoryView = new Uint8Array(instance.exports.mem.buffer);
        const byte = innerType == "num" ? 8 : 4;
        let [result, addr] = [[], value];
        for (let index = 0; index < length; index++) {
            const sliced = memoryView.slice(addr, addr + byte);
            const elem = concatBytes(sliced, byte == 8);
            result.push(read(instance, innerType, elem));
            addr += byte;
        }
        return result;
    } else if (type.type == "dict") {
        if (value == -1) return null;
        const [pointer, result] = [value, {}];
        const memoryView = new Uint8Array(instance.exports.mem.buffer);
        for (let [name, field] of Object.entries(type.fields)) {
            const address = pointer + field.offset;
            const value = (() => {
                if (field.type == "num") {
                    const sliced = memoryView.slice(address, address + 8);
                    return concatBytes(sliced, true);
                } else {
                    const sliced = memoryView.slice(address, address + 4);
                    return concatBytes(sliced);
                }
            })();
            result[name] = read(instance, field.type, value);
        }
        return result;
    } else if (type.type == "enum") {
        return type.enum[value];
    } else {
        return type;
    }
}

export function write(instance, type, value) {
    const buffer = instance.exports.mem.buffer;
    if (type == null) return null;
    else if (type === "int") return value;
    else if (type === "num") return value;
    else if (type === "str") {
        const utf8 = new TextEncoder().encode(value + "\0");
        const ptr = instance.exports.malloc(utf8.length);
        new Uint8Array(buffer, ptr, utf8.length).set(utf8);
        return ptr;
    } else if (type.type === "array") {
        const elemSize = type.element === "num" ? 8 : 4;
        const total = elemSize * value.length;
        const ptr = instance.exports.malloc(total);
        const view = new DataView(buffer, ptr, total);

        let array = [];
        for (elm of value) {
            array.push(write(instance, type.element, elm));
        }

        for (let i = 0; i < value.length; i++) {
            const off = i * elemSize;
            let method = type.element === "num" ? "setFloat64" : "setInt32";
            view[method](off, array[i], true);
        }
        return ptr;
    } else if (type.type == "dict") {
        let array = [];
        for (let [_name, field] of Object.entries(type.fields)) {
            array.push(write(instance, field.type, field));
        }
        const pointer = instance.exports.allocator;
        for (let [_name, field] of Object.entries(type.fields)) {
            const bytes = field.type == "num" ? 8 : 4;
            let addr = instance.exports.allocator;
            instance.exports.malloc(bytes);
            memory.set([field], addr);
        }
        return pointer;
    }
}

function concatBytes(bytes, is_64bit = false) {
    const buffer = new ArrayBuffer(8);
    const view = new DataView(buffer);
    let index = 0;
    for (let byte of bytes) {
        view.setUint8(index, byte);
        index += 1;
    }
    return is_64bit ? view.getFloat64(0, true) : view.getInt32(0, true);
}
