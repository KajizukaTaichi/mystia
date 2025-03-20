let sum(array, len) = {
    let idx = 0;
    let result = 0;
    while idx < len loop {
        let result = result + array[idx];
        let idx = idx + 1
    };
    result
};

let fuga = [100, 200, 300];
sum(fuga, 3)
