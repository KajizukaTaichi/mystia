let scalar_mul(matrix: [[num; 3]; 3], value: num) = {
    let i = 0;
    while i < 3 loop {
        let j = 0;
        while j < 3 loop {
            let matrix[i][j] = matrix[i][j] * value;
            let j = j + 1
        };
        let i = i + 1
    };
    matrix
};

let matrix = [
    [1.0, 2.0, 3.0],
    [4.0, 5.0, 6.0]
];
matrix[0][2]
