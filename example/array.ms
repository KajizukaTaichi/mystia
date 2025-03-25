let sum(array: [num], length: num) = {
    let index = 0;
    let result = 0;

    while index < length loop {
        let result = result + array[index];
        let index = index + 1
    };
    result
};

sum([100, 200, 300], 3)
