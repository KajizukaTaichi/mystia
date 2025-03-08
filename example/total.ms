fn total(start as int, end as int, step as float) as float = {
    declare result as float;
    let result = 0.0;
    while start < end loop {
        let result = result + start;
        let start = start + step
    };
    result
};

total(1, 10, 0.5)
