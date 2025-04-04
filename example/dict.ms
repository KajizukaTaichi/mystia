let alice = !{
    age: 13.2,
    score: !{
        math: 10,
        english: 95,
        science: 6
    },
    result: false
};
if alice.score.english >= 80 then {
    let alice.result = true
};

alice.age: int * alice.score.math
