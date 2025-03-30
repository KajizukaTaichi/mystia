let attended_days = [
    1, 5, 8, 12,
    15, 19, 22, 26
];
let scores = dict{
    math: 50,
    english: 10,
    science: 100
};
let alice = dict{
    age: 13,
    attended_days,
    exam: scores,
    result: false
};
if alice.exam.science >= 80 then {
    let alice.result = true
};

let alice.exam.english = 100;
alice.age * alice.exam.english
