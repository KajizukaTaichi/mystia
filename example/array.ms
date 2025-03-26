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

let foo = [1.2, 2.56, 3.14];
sum(foo, 64)
