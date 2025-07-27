const BYTES = 4;

export function read(instance, type, value) {
    if (type == null) return undefined;
    if (type == "int") {
        return value;
    } else if (type == "num") {
        return Math.round(value * 1e6) / 1e6;
    } else if (type == "bool") {
        return value != 0;
    } else if (type == "str") {
        if (value == -1) return null;
        let stringLength = value;
        const memoryView = new Uint8Array(instance.exports.mem.buffer);
        while (memoryView[stringLength] != 0) stringLength++;
        const stringBytes = memoryView.slice(value, stringLength);
        const textDecoder = new TextDecoder("utf-8");
        return textDecoder.decode(stringBytes);
    } else if (type.type == "array") {
        if (value == -1) return null;
        const innerType = type.element;
        let [result, addr] = [[], value + BYTES];
        const memoryView = new Uint8Array(instance.exports.mem.buffer);
        const length = concatBytes(memoryView.slice(value, addr), false);
        for (let index = 0; index < length; index++) {
            const sliced = memoryView.slice(addr, addr + BYTES);
            const elem = concatBytes(sliced, innerType == "num");
            result.push(read(instance, innerType, elem));
            addr += BYTES;
        }
        return result;
    } else if (type.type == "dict") {
        if (value == -1) return null;
        const [pointer, result] = [value, {}];
        const memoryView = new Uint8Array(instance.exports.mem.buffer);
        for (let [name, field] of Object.entries(type.fields)) {
            const address = pointer + field.offset;
            const sliced = memoryView.slice(address, address + BYTES);
            const value = concatBytes(sliced, field.type == "num");
            const fieldType = field.type.type == "alias" ? type : field.type;
            result[name] = read(instance, fieldType, value);
        }
        return result;
    } else if (type.type == "enum") {
        return type.enum[value];
    } else if (type.type == "alias") {
        return null;
    } else {
        return type;
    }
}

export function write(instance, type, value) {
    const reader = (type) => (type == "num" ? "setFloat32" : "setInt32");
    const buffer = instance.exports.mem.buffer;
    if (type == null) return null;
    else if (type == "int") return value;
    else if (type == "num") return value;
    else if (type == "str") {
        const utf8 = new TextEncoder().encode(value + "\0");
        const ptr = instance.exports.malloc(utf8.length);
        new Uint8Array(buffer, ptr, utf8.length).set(utf8);
        return ptr;
    } else if (type.type == "array") {
        let array = [];
        for (let elm of value) array.push(write(instance, type.element, elm));
        const size = BYTES * value.length + BYTES;
        const ptr = instance.exports.malloc(size);
        const view = new DataView(buffer, ptr, size);
        let addr = 0;
        view.setInt32(addr, value.length, true);
        addr += BYTES;
        for (let elm of array) {
            view[reader(type.element)](addr, elm, true);
            addr += BYTES;
        }
        return ptr;
    } else if (type.type == "dict") {
        for (let [name, field] of Object.entries(type.fields))
            type.fields[name] = write(instance, field.type, value[name]);
        const ptr = instance.exports.malloc(field.length * BYTES);
        let addr = ptr;
        for (let [_name, field] of Object.entries(type.fields)) {
            view[reader(type.element)](addr, field, true);
            addr += BYTES;
        }
        return ptr;
    }
}

export function concatBytes(bytes, is_float = false) {
    const buffer = new ArrayBuffer(8);
    const view = new DataView(buffer);
    let index = 0;
    for (let byte of bytes) {
        view.setUint8(index, byte);
        index += 1;
    }
    return is_float ? view.getFloat32(0, true) : view.getInt32(0, true);
}
