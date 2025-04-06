import { createInterface } from "readline";
import { mystia } from "./docs/node/main.js";

const rl = createInterface({
    input: process.stdin,
    output: process.stdout,
    prompt: "> ",
});

console.log("Mystia REPL");

// プロンプト表示
rl.prompt();

// ユーザーからの入力を受け取り、評価して表示する
rl.on("line", async (input) => {
    // 'exit' と入力された場合に終了
    if (input.trim() === "exit") {
        rl.close();
        return;
    }

    try {
        mystia(code).then((x) => console.log(x));
    } catch {
        console.error(
            "Error occurred during compilation. Check out if the program is correct",
        );
    }
    rl.prompt();
});

// REPLを閉じる時の処理
rl.on("close", () => {
    console.log("REPLを終了します。");
    process.exit(0);
});
