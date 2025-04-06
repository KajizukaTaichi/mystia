import { createInterface } from "readline";
import { mystia } from "./docs/node/main.js";

console.clear();
const rl = createInterface({
    input: process.stdin,
    output: process.stdout,
});

console.log("Mystia REPL");
function mainloop() {
    rl.question("You: ", async (code) => {
        try {
            console.log(`Mystia: ${await mystia(code)}`);
        } catch {
            console.error(
                "Error occurred during compilation. Check out if the program is correct",
            );
        }
        rl.close();
        mainloop();
    });
}
