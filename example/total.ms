fn total(start: int, end: int, step: float): float = {
    declare total: float;
    let total = 0;
    while start < end loop {
        let total = total + start;
        let start = start + step
    };
    total
};

total(1, 10, 0.5)
