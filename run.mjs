import { readFileSync } from "node:fs";
import { mystia } from "./docs/binding/node.mjs";

const args = process.argv.slice(2);
if (args.length === 0) {
    console.log("Mystia Runtime 2025 Node.js Edition");
    process.exit(1);
}

const filePath = args[0];
const source = readFileSync(filePath, "utf8");
mystia(source.toString());
