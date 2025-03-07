fn fact(n) {
    if n == 0 then 1
    else n * fact(n-1)
};

declare x;
let x = 2 * 3;
fact(x)
