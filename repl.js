import { createInterface } from "readline";
import { mystia } from "./docs/node/main.js";

const rl = createInterface({
    input: process.stdin,
    output: process.stdout,
    prompt: "> ",
});

console.clear();
console.log("Mystia REPL");

rl.prompt();

// ユーザーからの入力を受け取り、評価して表示する
rl.on("line", async (code) => {
    mystia(code)
        .then((x) => console.log(x))
        .then(() => rl.prompt());
});

// REPLを閉じる時の処理
rl.on("close", () => {
    console.log("REPLを終了します。");
    process.exit(0);
});
