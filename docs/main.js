import init, { mystia_compile } from "./pkg/mystia_wasm.js";

export async function mystia(mystiaCode) {
    await init();
    const watCode = mystia_compile(mystiaCode);
    const binary = await WebAssembly.compile(
        await (
            await fetch(`data:application/wasm;base64,${btoa(watCode)}`)
        ).arrayBuffer(),
    );

    const instance = await WebAssembly.instantiate(binary);
    if (instance.exports._start) {
        return instance.exports._start();
    } else {
        console.log('entry point "_start" function not found.');
    }
}

console.log(
    await (async function () {
        await mystia("1+2");
    })(),
);
