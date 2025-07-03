import { createInterface } from "readline";
import { mystia } from "./docs/runtime/node.mjs";

const rl = createInterface({
    input: process.stdin,
    output: process.stdout,
    prompt: "> ",
});

let code = "";

console.log("Mystia REPL");
rl.prompt();

rl.on("line", (input) => {
    if (input.trim() !== "")
        mystia(`${code};${input}`)
            .then((result) => {
                code += `;${input}`;
                if (result !== undefined) console.log(result);
            })
            .catch((e) => console.log("\u0007Error!", e))
            .then(() => rl.prompt());
    else rl.prompt();
});

rl.on("close", () => {
    console.log("Bye");
    process.exit(0);
});
