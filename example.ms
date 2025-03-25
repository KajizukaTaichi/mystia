let sum(array: [num], length: num) = {
    let index = 0;
    let result = 0;

    while index < length loop {
        let value = array[index];
        if value == 0 then {
            break
        };
        let result = result + value;
        let index = index + 1
    };
    result
};

sum([100, 200, 300], 64)
