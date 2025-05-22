import { write, read } from "./ffi.mjs";

export class MystiaStdLib {
    constructor() {
        this.functions = {
            to_str: (value) => {
                return write(this.instance, "str", value.toString());
            },
            to_num: (value) => {
                return parseFloat(read(this.instance, "str", value));
            },
            repeat: (value, count) => {
                return write(
                    this.instance,
                    "str",
                    read(this.instance, "str", value).repeat(count),
                );
            },
            concat: (str1, str2) => {
                str1 = read(this.instance, "str", str1);
                str2 = read(this.instance, "str", str2);
                return write(this.instance, "str", str1 + str2);
            },
            rand: () => Math.random(),
        };
    }
    set_wasm(instance) {
        this.instance = instance;
    }
    bridge() {
        return {
            to_str: (num) => this.functions.to_str(num),
            to_num: (str) => this.functions.to_num(str),
            concat: (str1, str2) => this.functions.concat(str1, str2),
            repeat: (str, count) => this.functions.repeat(str, count),
            rand: () => this.functions.rand(),
        };
    }
}

export class MystiaNodeLib extends MystiaStdLib {
    constructor() {
        super();
        this.functions.print = (message) => {
            console.log(read(this.instance, "str", message));
        };
    }
    bridge() {
        return {
            ...super.bridge(),
            ...{
                print: (ptr) => this.functions.print(ptr),
            },
        };
    }
}

let mystiaDomIndex = 0;
let getMystiaDom = (id) => `mystia-dom-${id}`;

export class MystiaWebLib extends MystiaStdLib {
    constructor() {
        super();
        this.functions.alert = (message) => {
            window.alert(read(this.instance, "str", message));
        };
        this.functions.confirm = (message) => {
            window.confirm(read(this.instance, "str", message));
        };
        this.functions.prompt = (message) => {
            const answer = window.prompt(read(this.instance, "str", message));
            return write(this.instance, "str", answer);
        };
        this.functions.init_canvas = () => {
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
        this.functions.draw = (x, y, color) => {
            const ctx = document
                .getElementById("mystia-canvas")
                .getContext("2d");
            const pallet = "white|black|grey|blue|violet|green|red|pink|yellow";
            const type = { type: "enum", enum: pallet.split("|") };
            ctx.fillStyle = read(this.instance, type, color);
            ctx.fillRect(x, y, 1, 1);
        };
        this.functions.new_elm = (tag, parent) => {
            const elm = document.createElement(read(this.instance, "str", tag));
            elm.setAttribute("id", getMystiaDom(mystiaDomIndex++));
            parent = document.getElementById(getMystiaDom(parent));
            if (parent === null) parent = document.body;
            parent.appendChild(elm);
            return mystiaDomIndex - 1;
        };
        this.functions.upd_elm = (id, property, content) => {
            property = read(this.instance, "str", property);
            content = read(this.instance, "str", content);
            let elm = document.getElementById(getMystiaDom(id));
            if (elm === null) elm = document.querySelector(id);
            if (property == "style") {
                elm.style.cssText += content;
            } else {
                elm[property] = content;
            }
        };
        this.functions.evt_elm = (id, name, func) => {
            const elm = document.getElementById(getMystiaDom(id));
            func = read(this.instance, "str", func);
            name = read(this.instance, "str", name);
            if (name.includes("key")) {
                document.body.addEventListener(name, (event) =>
                    this.instance.exports[func](event.keyCode),
                );
            } else {
                elm.addEventListener(name, () => this.instance.exports[func]());
            }
        };
    }
    bridge() {
        return {
            ...super.bridge(),
            ...{
                alert: (ptr) => this.functions.alert(ptr),
                confirm: (ptr) => this.functions.confirm(ptr),
                prompt: (ptr) => this.functions.prompt(ptr),
                init_canvas: () => this.functions.init_canvas(),
                draw: (x, y, color) => this.functions.draw(x, y, color),
                new_elm: (id, tag, parent) =>
                    this.functions.new_elm(id, tag, parent),
                upd_elm: (id, prop, content) =>
                    this.functions.upd_elm(id, prop, content),
                evt_elm: (id, name, func) =>
                    this.functions.evt_elm(id, name, func),
            },
        };
    }
}
