let add(a: T, b: T) = { let h = a; h + b };
let x = add@int(10, 20);
let y = add@num(1.2, 4.5);
x as num + y
