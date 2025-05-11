import { mystia } from "./binding/web.mjs";

class Mystia extends HTMLElement {
    constructor() {
        super();
        console.log("Welcome to the Mystia programming!");
    }

    async connectedCallback() {
        await mystia(this.innerHTML);
        this.innerHTML = "";
    }
}

customElements.define("mystia", Mystia);
