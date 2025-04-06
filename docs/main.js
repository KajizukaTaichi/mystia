import { mystia as compile } from "./node/mystia_wasm.js";

export async function mystia(code) {
    const compileResult = compile(code);
    const type = compileResult.get_return_type();
    const bytecodes = compileResult.get_bytecode().buffer;
    const { instance } = await WebAssembly.instantiate(bytecodes);
    const returns = instance.exports._start();
    if (type == "str") {
        let stringLength = returns;
        const memoryView = new Uint8Array(instance.exports.mem.buffer);
        while (memoryView[stringLength] !== 0) {
            stringLength++;
        }
        const stringBytes = memoryView.slice(returns, stringLength);
        const textDecoder = new TextDecoder("utf-8");
        return `"${textDecoder.decode(stringBytes)}"`;
    } else if (type == "int" || type == "num") {
        return returns.toString();
    } else if (type == "bool") {
        return returns == 1 ? "true" : "false";
    }
}
