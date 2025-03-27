import init, { mystia as compile } from "./pkg/mystia_wasm.js";

await init();

class MystiaValue {
    constructor(instance) {
        this.instance = instance;
    }

    getValue() {
        return this.instance.exports._start();
    }

    invoke(name) {
        return this.instance.exports[name]();
    }

    getString() {
        const address = this.getValue();
        const memoryView = new Uint8Array(this.instance.exports.mem.buffer);

        let stringLength = address;
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
    return new MystiaValue(instance);
}
