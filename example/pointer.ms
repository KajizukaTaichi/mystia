let swap(a as ptr, b as ptr) = {
    let temp = @a;
    let @a = @b;
    let @b = temp
};

let nums = [3, 7];
let a = nums;
let b = nums +1;
swap(a, b);
@a - @b
