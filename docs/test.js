const readline = require("readline");
const mystia = require("./node/main.js");

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
});

rl.question("何か入力してください: ", async (code) => {
    console.log(`あなたの入力: ${await mystia.mystia(code)}`);
    rl.close();
});
