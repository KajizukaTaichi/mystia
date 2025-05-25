export function module(
    importsInfo,
    customModules = {},
    MODULE_CLASSES,
    instances,
    importObject,
) {
    for (const { module, name, kind: _ } of importsInfo) {
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
            customModules[modName] ??
            instances[modName] ??
            new MODULE_CLASSES[modName]();
        if (!instanceObj) {
            throw new Error(`Unknown import module: ${modName}`);
        }
        const bridge = instanceObj.bridge();
        if (!(fnName in bridge)) {
            throw new Error(
                `Function ${fnName} not found in module ${modName}`,
            );
        }
        importObject.env[key] = bridge[fnName];
        instances[modName] = instanceObj;
    }
}
