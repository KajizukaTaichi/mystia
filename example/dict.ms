let alice = @{
    name: "Alice",
    age: 13.2,
    score: @{
        math: 10,
        english: 49,
        science: 87
    },
    total: 0,
    result: false
};

let exam = alice.score;
let alice.total = exam.math
    + exam.english
    + exam.science;

if alice.total >= 80 then {
    let alice.result = true
};

alice.score.math as num * alice.age
