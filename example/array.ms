let array = [1.5, 2.59, 3.14];
let length = 64;
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
