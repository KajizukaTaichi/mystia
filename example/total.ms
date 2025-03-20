let fact(number) = {
    let result = 1;
    while number > 0 loop {
        let result = result * number;
        let number = number - 1
    };
    result
};

fact(6)
