import { mystia as compile } from "../wasm/node/mystia_wasm.js";
import { MystiaWebLib } from "./lib.mjs";
import { MathLib } from "./math.mjs";
import { read } from "./ffi.mjs";

const MODULE_CLASSES = {
    MathLib,
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
          modName = "MystiaWebLib";
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
