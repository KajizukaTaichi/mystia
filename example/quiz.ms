load concat(a: str, b: str): str;
load prompt(msg: str): str;
load to_str(n: num): str;
load to_num(n: str): num;
load random.randint(a: int, b: int): int;

let a = randint(1, 9);
let b = randint(1, 9);
let sum = a * b;

let question = (b: str) + " x " + (a: str) + " = ?";

let result = prompt(question): int == sum;
if result then "Perfect!" else "Currect answer: " + (sum: str)
