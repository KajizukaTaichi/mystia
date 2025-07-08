import { mystia as compile } from "../wasm/node/mystia_wasm.js";
import { MystiaNodeLib } from "./lib/std.mjs";
import { MystiaMathLib } from "./lib/math.mjs";
import { MystiaOSLib } from "./lib/os.mjs";
import { MystiaRandomLib } from "./lib/random.mjs";
import { MystiaDatetimeLib } from "./lib/datetime.mjs";
import { MystiaTimeLib } from "./lib/time.mjs";
import { module } from "./module.mjs";
import { read } from "./ffi.mjs";

const moduleClasses = {
    math: MystiaMathLib,
    os: MystiaOSLib,
    random: MystiaRandomLib,
    datetime: MystiaDatetimeLib,
    time: MystiaTimeLib,
};

export async function mystia(code, customModules = {}) {
    const result = compile(code);
    console.log(result.get_return_type());
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
