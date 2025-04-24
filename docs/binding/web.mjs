import init, { mystia as compile } from "../web/mystia_wasm.js";
import { ffi } from "./ffi.mjs";

await init();
export async function mystia(code) {
    const result = compile(code);
    const type = eval(`(${result.get_return_type()})`);
    if (type == null) return null;
    const bytecodes = result.get_bytecode().buffer;

    let mystiaAlert = () => window.alert("[uninitialized]");
    let mystiaConfirm = () => window.confirm("[uninitialized]");
    let mystiaPrompt = () => window.prompt("[uninitialized]");
    const { instance } = await WebAssembly.instantiate(bytecodes, {
        env: {
            alert: (ptr) => mystiaAlert(ptr),
            confirm: (ptr) => mystiaConfirm(ptr),
            prompt: (ptr) => mystiaPrompt(ptr),
        },
    });
    mystiaAlert = (ptr) => window.alert(ffi(instance, "str", ptr));
    mystiaConfirm = (ptr) => window.confirm(ffi(instance, "str", ptr));
    mystiaPrompt = (ptr, defaultValue = "") => {
        const answer = window.prompt(ffi(instance, "str", ptr), defaultValue);
        const encoder = new TextEncoder();
        const utf8 = encoder.encode(answer + "\0");
        const str = instance.exports.alloc_index;
        const memory = new Uint8Array(instance.exports.mem.buffer);
        memory.set(utf8, str);
        return str;
    };

    const value = instance.exports._start();
    return ffi(instance, type, value);
}
