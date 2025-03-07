fn total(start, end) {
    declare total;
    let total = 0;
    while start < end loop {
        let total = total + start;
        let start = start + 1
    };
    total
};

total(1, 10)
