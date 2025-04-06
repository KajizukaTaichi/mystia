import { createInterface } from "readline";
import { mystia } from "./docs/node/main.js";

const rl = createInterface({
    input: process.stdin,
    output: process.stdout,
});

rl.question("何か入力してください: ", async (code) => {
    console.log(`あなたの入力: ${await mystia(code)}`);
    rl.close();
});
