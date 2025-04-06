import { mystia } from "./main.js";
async function main() {
    console.log(await mystia("let inc(n) = n + 1; inc(2)"));
}

main();
