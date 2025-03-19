let is_prime(n) = {
    if n < 2 then { return false };
    if n < 4 then { return true };
    if n % 2 == 0 then { return false };
    if n % 3 == 0 then { return false };

    let i = 5;
    while i * i < n loop {
        if n % i == 0 then {
            return false
        };
        let i = i + 6
    };
    true
};

is_prime(1234567891)
