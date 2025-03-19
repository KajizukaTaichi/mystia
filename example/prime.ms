let is_prime(n) = {
    let i = 2;
    while i < n loop {
        if n % i == 0 then {
            return false
        };
        let i = i + 1
    };
    true
};

is_prime(1234567891)
