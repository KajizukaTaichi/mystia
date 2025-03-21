let sum(array as ptr, length) = {
    let index = 0x00;
    let count = 0;
    let result = 0;

    while count < length loop {
        let result = result + array[index];
        let idx = index + 0x04;
        let index = index + 1
    };
    result
};

let fuga = [100, 200, 300];
sum(fuga, 3)
