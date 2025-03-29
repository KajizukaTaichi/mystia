let is_prime(n: int) = {
    if n < 2 then false
    else if n < 4 then true
    else if n % 2 == 0 then false
    else if n % 3 == 0 then false
    else {
        let i = 5;
        while i * i <= n loop {
            if n % i == 0 then {
                return false
            };
            let i = i + 6
        };
        true
    }
};

if is_prime(5987)
    then "just a prime number"
    else "not a prime number"
