let alice = !{
    age: 13,
    score: !{
        math: 10,
        english: 12,
        science: 6
    },
    result: false
};
if alice.score.math >= 80 then {
    let alice.result = true
};

alice.score.math
