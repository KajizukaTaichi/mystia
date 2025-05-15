load concat(a: str, b: str): str;
load prompt(msg: str): str;
load to_str(n: num): str;
load to_num(n: str): num;
load rand(): num;

let n_a = rand() * 10.0: int;
let n_b = rand() * 10.0: int;
let n_sum = n_a + n_b;

let s_a = n_a:num as str;
let s_b = n_b:num as str;
let s_sum = n_sum:num as str;
let question = s_a + " + " + s_b + " = ?";

let result = prompt(question): num: int == n_sum;
if result then "Perfect!" else "Currect answer: " + s_sum
