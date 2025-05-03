import init, { mystia as compile } from "../web/mystia_wasm.js";
import { write as mystia_write, read as mystia_read } from "./ffi.mjs";

await init();
export async function mystia(code) {
    const result = compile(code);
    const type = eval(`(${result.get_return_type()})`);
    const bytecodes = result.get_bytecode().buffer;
    if (type == null) return null;

    let mystiaFunctions = {
        alert: null,
        confirm: null,
        prompt: null,
        init_canvas: null,
        draw: null,
        int_to_str: null,
        concat: null,
        write: null,
    };
    const { instance } = await WebAssembly.instantiate(bytecodes, {
        env: {
            alert: (ptr) => mystiaFunctions.alert(ptr),
            confirm: (ptr) => mystiaFunctions.confirm(ptr),
            prompt: (ptr) => mystiaFunctions.prompt(ptr),
            init_canvas: () => mystiaFunctions.init_canvas(),
            draw: (x, y, color) => mystiaFunctions.draw(x, y, color),
            int_to_str: (num) => mystiaFunctions.int_to_str(num),
            concat: (str1, str2) => mystiaFunctions.concat(str1, str2),
            write: (data) => mystiaFunctions.write(data),
        },
    });
    mystiaFunctions.alert = (message) => {
        window.alert(mystia_read(instance, "str", message));
    };
    mystiaFunctions.confirm = (message) => {
        window.confirm(mystia_read(instance, "str", message));
    };
    mystiaFunctions.prompt = (message) => {
        const answer = window.prompt(mystia_read(instance, "str", message));
        return mystia_write(instance, "str", answer);
    };
    mystiaFunctions.init_canvas = () => {
        const canvas = document.getElementById("mystia-canvas");
        if (canvas == null) {
            canvas = document.createElement("canvas");
            canvas.width = window.innerWidth;
            canvas.height = window.innerHeight;
            canvas.id = "mystia-canvas";
            document.body.appendChild(canvas);
        } else {
            const ctx = canvas.getContext("2d");
            ctx.clearRect(0, 0, canvas.width, canvas.height);
        }
    };
    mystiaFunctions.draw = (x, y, color) => {
        const ctx = document.getElementById("mystia-canvas").getContext("2d");
        const type = {
            type: "enum",
            enum: [
                ["white", "black", "grey", "blue", "violet"],
                ["green", "red", "pink", "yellow"],
            ].flat(),
        };
        ctx.fillStyle = mystia_read(instance, type, color);
        ctx.fillRect(x, y, 1, 1);
    };
    mystiaFunctions.int_to_str = (value) => {
        return mystia_write(instance, "str", value.toString());
    };
    mystiaFunctions.concat = (str1, str2) => {
        const strs1 = mystia_read(instance, "str", str1);
        const strs2 = mystia_read(instance, "str", str2);
        return mystia_write(instance, "str", strs1 + strs2);
    };
    mystiaFunctions.write = (data) => {
        document.write(mystia_read(instance, "str", data));
    };

    const value = instance.exports._start();
    return mystia_read(instance, type, value);
}
