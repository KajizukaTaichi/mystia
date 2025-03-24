let swap(a: ptr, b: ptr) = {
    let tmp = @a;
    let @a = @b;
    let @b = tmp
};

let a = 0x00; let @a = 3;
let b = 0x04; let @b = 7;
swap(a, b);
@a - @b
