let add_matrices(X: [[num]], Y: [[num]], rows: int, cols: int) = {
    let result = [[0.0]];
    let i = 0;
    while i < rows loop {
        let new = [0.0];
        let j = 0;
        while j < cols loop {
            let new[j] = X[i][j] + Y[i][j];
            let j = j + 1
        };
        let result[i] = new;
        let i = i + 1
    };
    result
};

let r1 = [1.0, 2.0];
let r2 = [2.5, 5.0];
let m1 = [r1, r2];

let r1 = [1.0, 3.0];
let r2 = [6.0, 6.0];
let m2 = [r1, r2];

let m3 = add_matrices(m1, m2, 2, 2);
m3[1][1]
