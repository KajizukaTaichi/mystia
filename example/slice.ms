let slice3(a: [int; 10], n: int) = memcpy(a as int +(4*n) as [int;3]);
let foo = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let bar = slice3(foo, 3);
bar
