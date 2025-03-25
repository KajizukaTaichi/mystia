let sum(array: [num], length: num) = {
    let index = 0;
    let result = 0;

    while index < length loop {
        let result = result + array[index];
        let index = index + 1
    };
    result
};

let x = [1, 2, 3];
sum([200, sum(x, 3), 300], 3)
