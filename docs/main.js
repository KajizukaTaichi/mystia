import init, { mystia as compile } from "./pkg/mystia_wasm.js";

await init();

class MystiaValue {
    constructor(instance) {
        this.instance = instance;
    }

    get_value() {
        return instance.exports._start();
    }

    invoke(name) {
        return instance.exports[name]();
    }

    get_string() {
        const address = instance.exports._start();
        const memoryView = new Uint8Array(instance.exports.memory.buffer);

        const stringLength = address;
        while (memoryView[stringLength] !== 0) {
            stringLength++;
        }
        const stringBytes = memoryView.slice(address, stringLength);
        const textDecoder = new TextDecoder("utf-8");
        return textDecoder.decode(stringBytes);
    }
}

export async function mystia(code) {
    const bytecodes = compile(code).buffer;
    const { instance } = await WebAssembly.instantiate(bytecodes);
    return MystiaValue(instance);
}
