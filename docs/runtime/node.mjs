import { mystia as compile } from "../wasm/node/mystia_wasm.js";
import { MystiaNodeLib } from "./lib.mjs";
import { MathLib } from "./math.mjs";
import { OSLib } from "./os.mjs";
import { module } from "./module.mjs";
import { read } from "./ffi.mjs";

const MODULE_CLASSES = {
    MathLib,
    OSLib,
};

export async function mystia(code, customModules = {}) {
    const result = compile(code);
    const returnType = eval(`(${result.get_return_type()})`);
    const bytecodes = result.get_bytecode().buffer;
    const moduleObj = await WebAssembly.compile(bytecodes);
    const importsInfo = WebAssembly.Module.imports(moduleObj);
    const stdLib = new MystiaNodeLib();
    const importObject = { env: { ...stdLib.bridge() } };
    const instances = { MystiaNodeLib: stdLib };
    module(importsInfo, MODULE_CLASSES, customModules, instances, importObject);

    const wab = bytecodes;
    const { instance } = await WebAssembly.instantiate(wab, importObject);
    Object.values(instances).forEach((inst) => inst.set_wasm(instance));
    const raw = instance.exports._start();
    return read(instance, returnType, raw);
}
