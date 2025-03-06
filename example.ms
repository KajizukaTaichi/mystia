fn fact(n) {
    if n == 0 then 1
    else n * fact(n-1)
};

declare x;
let x = 1 + 2;
fact(x)
