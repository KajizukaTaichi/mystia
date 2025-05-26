export function module({
    importsInfo,
    moduleClasses,
    customModules = {},
    instances,
    importObject,
    runtime,
}) {
    for (const { module, name, kind: _ } of importsInfo) {
        if (module !== "env") continue;
        let modName, fnName, key;
        if (name.includes(".")) {
            [modName, fnName] = name.split(".");
            key = name;
        } else {
            modName = `Mystia${runtime}Lib`;
            fnName = name;
            key = fnName;
        }
        const instanceObj =
            customModules[modName] ??
            instances[modName] ??
            new moduleClasses[modName]();
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
