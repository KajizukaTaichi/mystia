import { createInterface } from "readline";
import { mystia } from "./docs/node/main.js";

const rl = createInterface({
    input: process.stdin,
    output: process.stdout,
    prompt: "> ",
});

let code = "";

console.clear();
console.log("Mystia REPL");
rl.prompt();

rl.on("line", (input) => {
    if (input.trim() !== "") {
        mystia(`${code};${input}`)
            .then((x) => {
                if (x === undefined) {
                    code += `;${input}`;
                } else {
                    console.log(x);
                }
            })
            .then(() => rl.prompt());
    } else {
        rl.prompt();
    }
});

rl.on("close", () => {
    console.log("Bye");
    process.exit(0);
});
