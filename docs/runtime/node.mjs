import { mystia as compile } from "../wasm/node/mystia_wasm.js";
import { MystiaNodeLib } from "./lib/std.mjs";
import { MathLib } from "./lib/math.mjs";
import { OSLib } from "./lib/os.mjs";
import { module } from "./module.mjs";
import { read } from "./ffi.mjs";

const moduleClasses = {
    math: MathLib,
    os: OSLib,
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
    module({
        importsInfo,
        moduleClasses,
        customModules,
        instances,
        importObject,
        runtime: "Node",
    });

    const wab = bytecodes;
    const { instance } = await WebAssembly.instantiate(wab, importObject);
    Object.values(instances).forEach((inst) => inst.set_wasm(instance));
    const raw = instance.exports._start();
    return read(instance, returnType, raw);
}
