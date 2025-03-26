let sum(array: [num], length: int) = {
    let index = 0;
    let result = 0.0;

    while index < length loop {
        let value = array[index];
        if value == 0.0 then {
            break
        };
        let result = result + value;
        let index = index + 1
    };
    result
};

let hoge = [1.5, 2.59, 3.14];
let hoge[0] = hoge[0] * 10;
sum(hoge, 64)
