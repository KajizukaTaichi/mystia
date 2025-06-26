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
        const innerType = type.element;
        const memoryView = new Uint8Array(instance.exports.mem.buffer);
        const byte = innerType == "num" ? 8 : 4;
        const length = concatBytes(memoryView.slice(addr, addr + 4), false);
        let [result, addr] = [[], value];
        for (let index = 0; index < length; index++) {
            const sliced = memoryView.slice(addr + 4, addr + 4 + byte);
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
        const ptr = instance.exports.allocator + 0;
        new Uint8Array(buffer, ptr, utf8.length).set(utf8);
        instance.exports.malloc(utf8.length);
        return ptr;
    } else if (type.type === "array") {
        let array = [];
        for (let elm of value) {
            array.push(write(instance, type.element, elm));
        }
        const elemSize = type.element === "num" ? 8 : 4;
        const ptr = instance.exports.allocator + 0;
        const view = new DataView(buffer, ptr, elemSize * type.length);
        for (let elm of array) {
            const addr = instance.exports.allocator - ptr;
            const method = elemSize === 8 ? "setFloat64" : "setInt32";
            instance.exports.malloc(elemSize);
            view[method](addr, elm, true);
        }
        return ptr;
    } else if (type.type == "dict") {
        for (let [name, field] of Object.entries(type.fields)) {
            type.fields[name] = write(instance, field.type, value[name]);
        }
        const ptr = instance.exports.allocator + 0;
        for (let [_name, field] of Object.entries(type.fields)) {
            const bytes = field.type == "num" ? 8 : 4;
            const addr = instance.exports.allocator - ptr;
            const method = bytes === 8 ? "setFloat64" : "setInt32";
            instance.exports.malloc(bytes);
            view[method](addr, field, true);
        }
        return ptr;
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
