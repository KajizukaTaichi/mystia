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
    const { instance } = await WebAssembly.instantiate(bytecodes, {
        web: {
            alert: (ptr) => mystiaAlert(ptr),
            confirm: (ptr) => mystiaConfirm(ptr),
        },
    });
    mystiaAlert = (ptr) => window.alert(ffi(instance, "str", ptr));
    mystiaConfirm = (ptr) => window.confirm(ffi(instance, "str", ptr));

    const value = instance.exports._start();
    return ffi(instance, type, value);
}
