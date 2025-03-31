let alice = dict{
    age: 13,
    exam: dict{
        math: 50,
        english: 18,
        science: 100
    },
    result: false
};
if alice.exam.science >= 80 then {
    let alice.result = true
};

alice.exam.english
