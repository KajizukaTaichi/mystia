let swap(a as ptr, b as ptr) = {
    let temp = @a;
    let @a = @b;
    let @b = temp
};

let @0x00 = 3; let a = 0x00;
let @0x01 = 7; let b = 0x01;
swap(a, b);
@a - @b
