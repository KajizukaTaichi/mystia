fn fact(n as float) as float = {
    if n <= 0.0 then 1.0
    else n * fact(n-1.0)
};

fact(6.1)
