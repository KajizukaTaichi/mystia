let is_prime(n: int) = {
    if n < 2 then false
    else if n < 4 then true
    else if (n % 2 == 0) || (n % 3 == 0) then false
    else {
        let i = 5;
        while i * i <= n loop {
            if n % i == 0 then {
                return false
            };
            let i + 6
        };
        true
    }
};

let n = 1234567891;
if is_prime(n)
    then "just a prime number"
    else "not a prime number"
