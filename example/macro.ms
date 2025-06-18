load print(_: str): void;

macro times(n, block) = {
    let i = 0;
    while i < n loop {
        block;
        let i + 1
    }
};

5.times({
    let message = "Hello, world!";
    print(message)
})
