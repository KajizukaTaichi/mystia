let scalar_mul(vector: [num; 3], dimension: int, value: num) = {
    let index = 0;
    while index < dimension loop {
        let vector[index] = vector[index] * value;
        let index = index + 1
    };
    vector
};

let vec_3d = [1.0, 2.0, 3.0];
let result = scalar_mul(vec_3d, 3, 1.5);
result[5]
