load alert(message: str): void;
load prompt(message: str): str;
load concat(a: str, b: str): str;

let name = prompt("What is your name?");
alert(concat("Hello, ", name)); name
