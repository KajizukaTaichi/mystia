import { write as write, read as read } from "./ffi.mjs";

export class MystiaStdLib {
    constructor() {
        this.functions = {
            to_str: (value) => {
                return write(this.instance, "str", value.toString());
            },
            to_num: (value) => {
                return parseFloat(read(this.instance, "str", value));
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
            rand: () => this.functions.rand(),
        };
    }
}
