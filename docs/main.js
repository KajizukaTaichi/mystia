import init, { Mystia } from "./pkg/mystia_wasm";

export async function mystia(mystiaCode) {
    const watCode = Mystia.compile(mystiaCode);

    // WATコードをバイナリにコンパイル
    const binary = await WebAssembly.compile(
        await (
            await fetch(`data:application/wasm;base64,${btoa(watCode)}`)
        ).arrayBuffer(),
    );

    // WebAssemblyインスタンスを作成
    const instance = await WebAssembly.instantiate(binary);

    // _start関数を呼び出す
    if (instance.exports._start) {
        const result = instance.exports._start();
        console.log("_start function executed.");
        return result;
    } else {
        console.log("_start function not found.");
    }
}
