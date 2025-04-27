import init, { mystia as compile } from "../web/mystia_wasm.js";
import { ffi } from "./ffi.mjs";

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
        sleep: null,
    };
    const { instance } = await WebAssembly.instantiate(bytecodes, {
        env: {
            alert: (ptr) => mystiaFunctions.alert(ptr),
            confirm: (ptr) => mystiaFunctions.confirm(ptr),
            prompt: (ptr) => mystiaFunctions.prompt(ptr),
            init_canvas: () => mystiaFunctions.init_canvas(),
            draw: (x, y, color) => mystiaFunctions.draw(x, y, color),
            sleep: (ms) => mystiaFunctions.sleep(ms),
        },
    });
    mystiaFunctions.alert = (ptr) => window.alert(ffi(instance, "str", ptr));
    mystiaFunctions.confirm = (ptr) =>
        window.confirm(ffi(instance, "str", ptr));
    mystiaFunctions.prompt = (ptr) => {
        const answer = window.prompt(ffi(instance, "str", ptr));
        const utf8 = new TextEncoder().encode(answer + "\0");
        const str = instance.exports.alloc_index;
        const memory = new Uint8Array(instance.exports.mem.buffer);
        memory.set(utf8, str);
        return str;
    };
    mystiaFunctions.init_canvas = () => {
        let canvas = document.getElementById("mystia-canvas");
        if (canvas == null) {
            canvas = document.createElement("canvas");
            [canvas.width, canvas.height] = [500, 500];
            canvas.id = "mystia-canvas";
            document.body.appendChild(canvas);
        }
        canvas.getContext("2d").clearRect(0, 0, canvas.width, canvas.height);
    };
    mystiaFunctions.draw = (x, y, color) => {
        const ctx = document.getElementById("mystia-canvas").getContext("2d");
        ctx.fillStyle = ffi(
            instance,
            {
                type: "enum",
                enum: [
                    "white",
                    "black",
                    "grey",
                    "blue",
                    "violet",
                    "green",
                    "red",
                    "pink",
                    "yellow",
                ],
            },
            color,
        );
        ctx.fillRect(x, y, 1, 1);
    };
    mystiaFunctions.sleep = (ms) => {
        const end = Date.now() + ms;
        while (Date.now() < end) {
            // sleep in
        }
    };

    const value = instance.exports._start();
    return ffi(instance, type, value);
}
