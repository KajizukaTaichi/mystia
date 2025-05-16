import init, { mystia as compile } from "../wasm/web/mystia_wasm.js";
import { MystiaWebLib } from "./lib.mjs";

let mystiaDomIndex = 0;
let getMystiaDom = (id) => `mystia-dom-${id}`;

await init();
export async function mystia(code) {
    const result = compile(code);
    const type = eval(`(${result.get_return_type()})`);
    const bytecodes = result.get_bytecode().buffer;
    if (type == null) return null;

    let mystiaStdLib = new MystiaWebLib();
    const { instance } = await WebAssembly.instantiate(bytecodes, {
        env: mystiaStdLib.bridge(),
    });
    mystiaStdLib.set_wasm(instance);

    const value = instance.exports._start();
    return read(instance, type, value);
}
