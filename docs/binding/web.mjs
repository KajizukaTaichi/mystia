import init, { mystia as compile } from "../wasm/web/mystia_wasm.js";
import { write as write, read as read } from "./ffi.mjs";

await init();
export async function mystia(code) {
    const result = compile(code);
    const type = eval(`(${result.get_return_type()})`);
    const bytecodes = result.get_bytecode().buffer;
    if (type == null) return null;

    let libFuncs = {
        alert: null,
        confirm: null,
        prompt: null,
        init_canvas: null,
        draw: null,
        int_to_str: null,
        concat: null,
        rand: null,
        new_elm: null,
        upd_elm: null,
        evt_elm: null,
    };
    const { instance } = await WebAssembly.instantiate(bytecodes, {
        env: {
            alert: (ptr) => libFuncs.alert(ptr),
            confirm: (ptr) => libFuncs.confirm(ptr),
            prompt: (ptr) => libFuncs.prompt(ptr),
            init_canvas: () => libFuncs.init_canvas(),
            draw: (x, y, color) => libFuncs.draw(x, y, color),
            int_to_str: (num) => libFuncs.int_to_str(num),
            concat: (str1, str2) => libFuncs.concat(str1, str2),
            rand: () => libFuncs.rand(),
            new_elm: (id, tag, parent) => libFuncs.new_elm(id, tag, parent),
            upd_elm: (id, prop, content) => libFuncs.upd_elm(id, prop, content),
            evt_elm: (id, name, func) => libFuncs.evt_elm(id, name, func),
        },
    });
    libFuncs.alert = (message) => {
        window.alert(read(instance, "str", message));
    };
    libFuncs.confirm = (message) => {
        window.confirm(read(instance, "str", message));
    };
    libFuncs.prompt = (message) => {
        const answer = window.prompt(read(instance, "str", message));
        return write(instance, "str", answer);
    };
    libFuncs.init_canvas = () => {
        let canvas = document.getElementById("mystia-canvas");
        if (canvas == null) {
            canvas = document.createElement("canvas");
            canvas.width = window.innerWidth;
            canvas.height = window.innerHeight;
            canvas.style.width = `${window.innerWidth}px`;
            canvas.style.height = `${window.innerHeight}px`;
            canvas.id = "mystia-canvas";
            document.body.appendChild(canvas);
        } else {
            const ctx = canvas.getContext("2d");
            ctx.clearRect(0, 0, canvas.width, canvas.height);
        }
    };
    libFuncs.draw = (x, y, color) => {
        const ctx = document.getElementById("mystia-canvas").getContext("2d");
        const pallet = "white|black|grey|blue|violet|green|red|pink|yellow";
        const type = { type: "enum", enum: pallet.split("|") };
        ctx.fillStyle = read(instance, type, color);
        ctx.fillRect(x, y, 1, 1);
    };
    libFuncs.int_to_str = (value) => {
        return write(instance, "str", value.toString());
    };
    libFuncs.concat = (str1, str2) => {
        const strs1 = read(instance, "str", str1);
        const strs2 = read(instance, "str", str2);
        return write(instance, "str", strs1 + strs2);
    };
    libFuncs.rand = () => {
        return Math.random();
    };
    libFuncs.new_elm = (id, tag, parent) => {
        const elm = document.createElement(read(instance, "str", tag));
        elm.setAttribute("id", read(instance, "str", id));
        parent = document.getElementById(read(instance, "str", parent));
        if (parent === null) parent = document.body;
        parent.appendChild(elm);
    };
    libFuncs.upd_elm = (id, property, content) => {
        const elm = document.getElementById(read(instance, "str", id));
        elm[read(instance, "str", property)] = read(instance, "str", content);
    };
    libFuncs.evt_elm = (id, name, func) => {
        const elm = document.getElementById(read(instance, "str", id));
        func = read(instance, "str", func);
        name = read(instance, "str", name);
        if (name.includes("key")) {
            elm.addEventListener(name, function (event) {
                instance.exports[func](write(instance, "str", event.key));
            });
        } else {
        }
    };

    const value = instance.exports._start();
    return read(instance, type, value);
}
