import { mystia } from "./runtime/web.mjs";
let timer;

const codeEditor = document.getElementById("code");
const resultArea = document.getElementById("result");
const runBtn = document.getElementById("run");
const buildBtn = document.getElementById("build");

const timerLabel = document.getElementById("timer");
const exampleFieldInput = document.getElementById("example-field");
const exampleSelectBtn = document.getElementById("example-select");

exampleSelectBtn.addEventListener("click", async () => {
    const exampleCodeUrl = `https://raw.githubusercontent.com/KajizukaTaichi/mystia/refs/heads/master/example/${exampleFieldInput.value}.ms`;
    const response = await fetch(exampleCodeUrl);
    codeEditor.value = await response.text();
});

runBtn.addEventListener("click", async () => {
    resultArea.innerHTML = "";
    timerLabel.textContent = "";
    const startTime = Date.now();
    timer = setInterval(() => {
        timerLabel.textContent = `Time: ${Date.now() - startTime}ms`;
    }, 1);
    // try {
    const result = await mystia(codeEditor.value);
    if (result !== undefined) {
        resultArea.innerHTML = JSON.stringify(result, null, 2);
    }
    // } catch (error) {
    // resultArea.innerHTML = error;
    // }
    clearInterval(timer);
});

buildBtn.addEventListener("click", () => {
    const data = `
        <!DOCTYPE html>
        <html>
            <head>
                <title>Mystia App</title>
            </head>
            <body>
                <script type="module" src="https://kajizukataichi.github.io/mystia/runtime/web.mjs"></\script>
                <mystia-code>${codeEditor.value}</mystia-code>
            </body>
        </html>
    `;
    const blob = new Blob([data], { type: "text/plain" });
    const link = document.createElement("a");
    link.href = URL.createObjectURL(blob);
    link.download = "mystia-app.html";
    link.click();
});
