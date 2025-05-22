let fact(n: int): int = {
    if n == 0
        then 1
        else n * fact(n - 1)
};

fact(5)
