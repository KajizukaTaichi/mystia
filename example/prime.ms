let is_prime(n: int) = {
    let i = 2;
    if n < i then {
        return false
    };
    while i * i <= n loop {
        if n % i == 0 then {
            return false
        };
        let i = i + 1
    };
    true
};

if is_prime(5987)
    then "just a prime number"
    else "not a prime number"
