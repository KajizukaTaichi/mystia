import init, { mystia as compile } from "../wasm/web/mystia_wasm.js";
import { MystiaWebLib } from "./lib/std.mjs";
import { MystiaMathLib } from "./lib/math.mjs";
import { MystiaRandomLib } from "./lib/random.mjs";
import { MystiaDatetimeLib } from "./lib/datetime.mjs";
import { MystiaTimeLib } from "./lib/time.mjs";
import { module } from "./module.mjs";
import { read } from "./ffi.mjs";

const moduleClasses = {
    math: MystiaMathLib,
    random: MystiaRandomLib,
    datetime: MystiaDatetimeLib,
    time: MystiaTimeLib,
};

await init();
export async function mystia(code, customModules = {}) {
    const result = compile(code);
    const returnType = eval(`(${result.get_return_type()})`);
    const bytecodes = result.get_bytecode().buffer;
    const moduleObj = await WebAssembly.compile(bytecodes);
    const importsInfo = WebAssembly.Module.imports(moduleObj);
    const stdLib = new MystiaWebLib();
    const importObject = { env: { ...stdLib.bridge() } };
    const instances = { MystiaWebLib: stdLib };
    module({
        importsInfo,
        moduleClasses,
        customModules,
        instances,
        importObject,
        runtime: "Web",
    });

    const wab = bytecodes;
    const { instance } = await WebAssembly.instantiate(wab, importObject);
    Object.values(instances).forEach((inst) => inst.set_wasm(instance));
    const raw = instance.exports._start();
    if (returnType != null) {
        return read(instance, returnType, raw);
    }
}

class Mystia extends HTMLElement {
    constructor() {
        super();
        console.log("Welcome to the Mystia programming!");
    }

    async connectedCallback() {
        await mystia(this.innerHTML);
        this.remove();
    }
}

customElements.define("mystia-code", Mystia);
