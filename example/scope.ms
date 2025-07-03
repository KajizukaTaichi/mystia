let a = {
    let b = {
        let inc(n: int) = n + 1;
        let c = {
            let d = 1;
            d.inc()
        };
        inc(c) + 1
    };
    b + 1
};
a + 1
