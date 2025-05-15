load concat(a: str, b: str): str;
load prompt(msg: str): str;
load to_str(n: num): str;
load to_num(n: str): num;
load rand(): num;

let a = rand() * 10.0: int + 1;
let b = rand() * 10.0: int + 1;
let sum = n_a + n_b;

let question = (a: str) + " + " + (b: str) + " = ?";

let result = prompt(question): num: int == n_sum;
if result then "Perfect!" else "Currect answer: " + (sum: str)
