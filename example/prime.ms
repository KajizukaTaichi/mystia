let is_prime(n: int) = {
    let i = 2;
    while i * i < n loop {
        if n % i == 0 then {
            return false
        };
        let i = i + 1
    };
    true
};

is_prime(1234567891)
