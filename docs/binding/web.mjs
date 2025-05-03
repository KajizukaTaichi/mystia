import init, { mystia as compile } from "../web/mystia_wasm.js";
import { write, read } from "./ffi.mjs";

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
    };
    const { instance } = await WebAssembly.instantiate(bytecodes, {
        env: {
            alert: (ptr) => mystiaFunctions.alert(ptr),
            confirm: (ptr) => mystiaFunctions.confirm(ptr),
            prompt: (ptr) => mystiaFunctions.prompt(ptr),
            init_canvas: () => mystiaFunctions.init_canvas(),
            draw: (x, y, color) => mystiaFunctions.draw(x, y, color),
        },
    });
    mystiaFunctions.alert = (message) => {
        window.alert(read(instance, "str", message));
    };
    mystiaFunctions.confirm = (message) => {
        window.confirm(read(instance, "str", message));
    };
    mystiaFunctions.prompt = (message) => {
        const answer = window.prompt(read(instance, "str", message));
        return write(instance, "str", answer);
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
        ctx.fillStyle = read(instance, type, color);
        ctx.fillRect(x, y, 1, 1);
    };

    const value = instance.exports._start();
    return read(instance, type, value);
}
