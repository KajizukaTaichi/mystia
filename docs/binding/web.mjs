import init, { mystia as compile } from "../web/mystia_wasm.js";
import { ffi } from "./ffi.mjs";

await init();
export async function mystia(code) {
    const result = compile(code);
    const type = eval(`(${result.get_return_type()})`);
    const bytecodes = result.get_bytecode().buffer;
    if (type == null) return null;

    let mystiaAlert, mystiaConfirm, mystiaPrompt, mystiaDraw;
    const { instance } = await WebAssembly.instantiate(bytecodes, {
        env: {
            alert: mystiaAlert,
            confirm: mystiaConfirm,
            prompt: mystiaPrompt,
            draw: mystiaDraw,
        },
    });
    mystiaAlert = (ptr) => window.alert(ffi(instance, "str", ptr));
    mystiaConfirm = (ptr) => window.confirm(ffi(instance, "str", ptr));
    mystiaPrompt = (ptr) => {
        const answer = window.prompt(ffi(instance, "str", ptr));
        const utf8 = new TextEncoder().encode(answer + "\0");
        const str = instance.exports.alloc_index;
        const memory = new Uint8Array(instance.exports.mem.buffer);
        memory.set(utf8, str);
        return str;
    };
    mystiaDraw = (x, y, color) => {
        const ctx = document.getElementById("mystia-canvas").getContext("2d");
        ctx.fillStyle = ffi(instance, "str", color);
        ctx.fillRect(x * 100, y * 100, 100, 100);
    };

    console.log(instance.exports.draw);
    const value = instance.exports._start();
    return ffi(instance, type, value);
}
