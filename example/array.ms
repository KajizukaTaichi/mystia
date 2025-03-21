let sum(array as ptr, length) = {
    let index = 0x00;
    let count = 0;
    let result = 0;

    while count < length loop {
        let result = result + array[index];
        let index = index + 0x04;
        let count = count + 1
    };
    result
};

sum([100, 200, 300], 3)
