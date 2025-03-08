fn total(start as float, end as float, step as float) as float = {
    declare result as float;
    let result = 0.0;
    while start < end loop {
        let result = result + start;
        let start = start + step
    };
    result
};

total(0, 1 as float, 0.1)
