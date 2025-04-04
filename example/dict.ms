let alice = !{
    age: 13.2,
    score: !{
        math: 10,
        english: 9,
        science: 6,
    },
    total: 0,
    result: false
};

let score = alice.score;
let alice.total = score.math + score.english + score.science;

if alice.total >= 80 then {
    let alice.result = true
};

alice.total: num + alice.age
