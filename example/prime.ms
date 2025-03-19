let sqrt(n) = {
    let i = 0;
    while i * i <= n loop {
        let i = i + 1
    };
    i - 1
};

let is_prime(n) = {
    let i = 2;
    while i < sqrt(n) loop {
        if n % i == 0 then {
            return false
        };
        let i = i + 1
    };
    true
};

is_prime(1234567891)
