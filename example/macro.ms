load print(_: str): void;
macro repeat(n, block) = {
    let i = 0;
    while i < n loop {
        block;
        let i + 1
    }
};
repeat(10, print("Hello!"))
