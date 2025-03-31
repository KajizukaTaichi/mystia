let scores = dict{
    math: 50,
    english: 10,
    science: 100
};
let alice = dict{
    age: 13,
    exam: scores,
    result: false
};
if alice.exam.science >= 80 then {
    let alice.result = true
};

let alice.exam.english = 100;
alice.age * alice.exam.english
