fn total(number as float) as int = {
    let count = 1;
    let result = 0;

    while count as float < number loop {
        let result = result + count;
        let count = count + 1
    };
    result
};

total(10.0)
