import { createInterface } from "readline";
import { mystia } from "./docs/node/main.js";

console.clear();
const rl = createInterface({
    input: process.stdin,
    output: process.stdout,
});

console.log("Mystia REPL");
rl.question("You: ", async (code) => {
    console.log(`Mystia: ${await mystia(code)}`);
    rl.close();
});
