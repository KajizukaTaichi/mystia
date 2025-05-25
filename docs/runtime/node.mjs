import { mystia as compile } from "../wasm/node/mystia_wasm.js";
import { MystiaNodeLib } from "./lib.mjs";
import { MathLib } from "./math.mjs";
import { OSLib } from "./os.mjs";
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

  const moduleNames = new Set(
    importsInfo
      .filter(i => i.module === "env" && i.kind === "func")
      .map(i => i.name.split(".")[0])
  );

  for (const { module, name, kind } of importsInfo) {
    if (module !== "env") continue;
    let modName, fnName, key;
    if (name.includes(".")) {
        [modName, fnName] = name.split(".");
        key = name;
    } else {
        modName = "MystiaNodeLib";
        fnName = name;
        key = fnName;
    }
    const instanceObj =
      customModules[modName] ?? instances[modName] ?? (MODULE_CLASSES[modName] && new MODULE_CLASSES[modName]());
    if (!instanceObj) {
      throw new Error(`Unknown import module: ${modName}`);
    }
    const bridge = instanceObj.bridge();
    if (!(fnName in bridge)) {
        throw new Error(`Function ${fnName} not found in module ${modName}`);
    }
    importObject.env[key] = bridge[fnName];
    instances[modName] = instanceObj;
  }
  const wab = bytecodes;
  const { instance } = await WebAssembly.instantiate(wab, importObject);
  Object.values(instances).forEach(inst => inst.set_wasm(instance));
  const raw = instance.exports._start();
  return read(instance, returnType, raw);
}