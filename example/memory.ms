let swap(a as ptr, b as ptr) = {
    let temp = @a;
    let @a = @b;
    let @b = temp
};

let @0 = 3; let a = 0 as ptr;
let @1 = 7; let b = 1 as ptr;
swap(a, b);
@a - @b
