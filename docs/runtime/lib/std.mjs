import { write, read } from "../ffi.mjs";

export class MystiaStdLib {
    constructor() {
        this.functions = {
            to_str: (value, typ) => {
                return write(
                    this.instance,
                    "str",
                    read(
                        this.instance,
                        eval(
                            `(${read(this.instance, "str", typ).toString()})`,
                        ),
                        value,
                    ).toString(),
                );
            },
            to_num: (value, typ) => {
                return parseFloat(
                    read(
                        this.instance,
                        eval(
                            `(${read(this.instance, "str", typ).toString()})`,
                        ),
                        value,
                    ),
                );
            },
            repeat: (value, value_typ, count, count_typ) => {
                return write(
                    this.instance,
                    "str",
                    read(
                        this.instance,
                        eval(
                            `(${read(this.instance, "str", value_typ).toString()})`,
                        ),
                        value,
                    ).repeat(
                        read(
                            this.instance,
                            eval(
                                `(${read(this.instance, "str", count_typ).toString()})`,
                            ),
                            count,
                        ),
                    ),
                );
            },
            concat: (str1, str1_typ, str2, str2_typ) => {
                return write(
                    this.instance,
                    "str",
                    read(
                        this.instance,
                        eval(
                            `(${read(this.instance, "str", str1_typ).toString()})`,
                        ),
                        str1,
                    ) +
                    read(
                        this.instance,
                        eval(
                            `(${read(this.instance, "str", str2_typ).toString()})`,
                        ),
                        str2,
                    ),
                );
            },
            strcmp: (str1, str1_typ, str2, str2_typ) => {
                return (
                    read(
                        this.instance,
                        eval(
                            `(${read(this.instance, "str", str1_typ).toString()})`,
                        ),
                        str1,
                    ) ===
                    read(
                        this.instance,
                        eval(
                            `(${read(this.instance, "str", str2_typ).toString()})`,
                        ),
                        str2,
                    )
                );
            },
            strlen: (str, typ) => {
                return (
                    read(
                        this.instance,
                        eval(
                            `(${read(this.instance, "str", typ).toString()})`,
                        ),
                        str,
                    )
                ).length;
            },
            split: (str, str_typ, delimiter, delimiter_typ) => {
                const s = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", str_typ).toString()})`,
                    ),
                    str,
                );
                const d = read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", delimiter_typ).toString()})`,
                    ),
                    delimiter,
                );
                const index = s.indexOf(d);
                const splitted = [
                    s.substring(0, index),
                    s.substring(index + d.length),
                ];
                const typ = { type: "array", element: "str", length: 2 };
                return write(this.instance, typ, splitted);
            },
        };
    }
    set_wasm(instance) {
        this.instance = instance;
    }
    bridge() {
        const b = {};
        for (const k of Object.keys(this.functions)) {
            b[k] = (...a) => this.functions[k](...a);
        }
        return b;
    }
}

export class MystiaNodeLib extends MystiaStdLib {
    constructor() {
        super();
        this.functions.print = (message, message_typ) => {
            console.log(
                read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", message_typ).toString()})`,
                    ),
                    message,
                ),
            );
        };
    }
}

let mystiaDomIndex = 0;
let getMystiaDom = (id) => `mystia-dom-${id}`;

export class MystiaWebLib extends MystiaStdLib {
    constructor() {
        super();
        this.functions.alert = (message, message_typ) => {
            window.alert(
                read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", message_typ).toString()})`,
                    ),
                    message,
                ),
            );
        };
        this.functions.confirm = (message, message_typ) => {
            return window.confirm(
                read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", message_typ).toString()})`,
                    ),
                    message,
                ),
            );
        };
        this.functions.prompt = (message, message_typ) => {
            const answer = window.prompt(
                read(
                    this.instance,
                    eval(
                        `(${read(this.instance, "str", message_typ).toString()})`,
                    ),
                    message,
                ),
            );
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
        this.functions.draw = (x, x_typ, y, y_typ, color, color_typ) => {
            const X = read(
                this.instance,
                eval(
                    `(${read(this.instance, "str", x_typ).toString()})`,
                ),
                x,
            );
            const Y = read(
                this.instance,
                eval(
                    `(${read(this.instance, "str", y_typ).toString()})`,
                ),
                y,
            );
            const C = read(
                this.instance,
                eval(
                    `(${read(this.instance, "str", color_typ).toString()})`,
                ),
                color,
            );
            const ctx = document
                .getElementById("mystia-canvas")
                .getContext("2d");
            ctx.fillStyle = C;
            ctx.fillRect(X, Y, 1, 1);
        };
        this.functions.new_elm = (tag, tag_typ, parent, parent_typ) => {
            const T = read(
                this.instance,
                eval(
                    `(${read(this.instance, "str", tag_typ).toString()})`,
                ),
                tag,
            );
            const elm = document.createElement(T);
            elm.setAttribute("id", getMystiaDom(mystiaDomIndex++));
            let P = read(
                this.instance,
                eval(
                    `(${read(this.instance, "str", parent_typ).toString()})`,
                ),
                parent,
            );
            let parentElem = document.getElementById(getMystiaDom(P));
            if (parentElem === null) parentElem = document.body;
            parentElem.appendChild(elm);
            return mystiaDomIndex - 1;
        };
        this.functions.upd_elm = (id, id_typ, property, property_typ, content, content_typ) => {
            const I = read(
                this.instance,
                eval(
                    `(${read(this.instance, "str", id_typ).toString()})`,
                ),
                id,
            );
            const prop = read(
                this.instance,
                eval(
                    `(${read(this.instance, "str", property_typ).toString()})`,
                ),
                property,
            );
            const cont = read(
                this.instance,
                eval(
                    `(${read(this.instance, "str", content_typ).toString()})`,
                ),
                content,
            );
            let elm = document.getElementById(getMystiaDom(I));
            if (elm === null) elm = document.querySelector(id);
            if (prop == "style") {
                elm.style.cssText += cont;
            } else {
                elm[prop] = cont;
            }
        };
        this.functions.evt_elm = (id, id_typ, name, name_typ, func, func_typ) => {
            const I = read(
                this.instance,
                eval(
                    `(${read(this.instance, "str", id_typ).toString()})`,
                ),
                id,
            );
            const N = read(
                this.instance,
                eval(
                    `(${read(this.instance, "str", name_typ).toString()})`,
                ),
                name,
            );
            const F = read(
                this.instance,
                eval(
                    `(${read(this.instance, "str", func_typ).toString()})`,
                ),
                func,
            );
            const elm = document.getElementById(getMystiaDom(I));
            if (N.includes("key")) {
                document.body.addEventListener(N, (event) =>
                    this.instance.exports[F](event.keyCode),
                );
            } else {
                elm.addEventListener(N, () => this.instance.exports[F]());
            }
        };
    }
}
