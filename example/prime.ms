let n = 1234567891;

let i = 2;
while i * i <= n loop {
    if n % i == 0 then {
        return false
    };
    let i = i + 1
};
true
