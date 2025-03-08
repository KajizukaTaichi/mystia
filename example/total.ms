fn total(number as int) as int = {
    declare count as int;
    declare result as int;
    let count = 1;
    let result = 0;

    while count < number loop {
        let result = result + count;
        let count = count + 1
    };
    result
};

total(10)
