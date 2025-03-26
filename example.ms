let sum(array: [int], length: int) = {
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

let hoge = [100, 200, 300];
let hoge[0] = hoge[0] * 10;
sum(hoge, 64)
