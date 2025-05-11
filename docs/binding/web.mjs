import init, { mystia as compile } from "../wasm/web/mystia_wasm.js";
import { write as write, read as read } from "./ffi.mjs";

await init();

let appStatus = {};

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
        rand: null,
        new_elm: null,
        set_elm: null,
        tap_elm: null,
        get_status: null,
        set_status: null,
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
            rand: () => mystiaFunctions.rand(),
            new_elm: (id, tag) => mystiaFunctions.new_elm(id, tag),
            set_elm: (id, cotent) => mystiaFunctions.set_elm(id, cotent),
            tap_elm: (id, funcname) => mystiaFunctions.tap_elm(id, funcname),
            get_status: (id) => mystiaFunctions.get_status(id),
            set_status: (id, val) => mystiaFunctions.set_status(id, val),
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
    mystiaFunctions.int_to_str = (value) => {
        return write(instance, "str", value.toString());
    };
    mystiaFunctions.concat = (str1, str2) => {
        const strs1 = read(instance, "str", str1);
        const strs2 = read(instance, "str", str2);
        return write(instance, "str", strs1 + strs2);
    };
    mystiaFunctions.write = (data) => {
        document.write(read(instance, "str", data));
    };
    mystiaFunctions.rand = () => {
        return Math.random();
    };
    mystiaFunctions.new_elm = (id, tag) => {
        const elm = document.createElement(read(instance, "str", tag));
        elm.setAttribute("id", read(instance, "str", id));
        document.body.appendChild(elm);
    };
    mystiaFunctions.set_elm = (id, content) => {
        const elm = document.getElementById(read(instance, "str", id));
        elm.innerHTML = read(instance, "str", content);
    };
    mystiaFunctions.tap_elm = (id, funcname) => {
        const elm = document.getElementById(id);
        elm.onclick = function () {
            instance.exports[read(instance, "str", funcname)]();
        };
    };
    mystiaFunctions.get_status = (id) => {
        return appStatus[read(instance, "str", id)];
    };
    mystiaFunctions.set_status = (id, val) => {
        appStatus[read(instance, "str", id)] = val;
    };

    const value = instance.exports._start();
    return read(instance, type, value);
}
