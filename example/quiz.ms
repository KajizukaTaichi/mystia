load concat(a: str, b: str): str;
load prompt(msg: str): str;
load to_str(n: num): str;
load to_num(n: str): num;
load rand(): num;

let a = rand() * 8.0: int + 1;
let b = rand() * 8.0: int + 1;
let sum = a * b;

let a_ = a: str; let b_ = b as str;
let question =  a_ + " x " + b_ + " = ?";

let result = prompt(question): int == sum;
if result then "Perfect!" else "Currect answer: " + (sum: str)
