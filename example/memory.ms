let swap(a as ptr, b as ptr) = {
    let temp = @a;
    let @a = @b;
    let @b = temp
};

let a = 0x00; let @a = 3;
let b = 0x04; let @b = 7;
swap(a, b);
@a - @b
