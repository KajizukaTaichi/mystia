fn swap(a as int, b as int) as int = {
    let temp = &a;
    let &a = &b;
    let &b = temp;
};

let nums = [3, 7];
let a = 0;
let b = 1;
swap(a, b);
&a - &b
