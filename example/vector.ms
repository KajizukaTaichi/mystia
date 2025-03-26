let scalar_mul(vector: [num], dimension: int, value: num) = {
    let index = 0;
    while index < dimension loop {
        let vector[index] = vector[index] * value;
        let index = index + 1
    };
    vector
};

let result = scalar_mul([1.0, 2.0, 3.0], 3, 1.5);
result[1]
