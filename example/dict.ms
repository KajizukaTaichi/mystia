let profile = dict{
    let age = 13;
    let fuga = true
};
let b = [1, 2];
let members = dict{
    let alice = b;
    let bob = profile
};

let members.alice[0] = 100;
members.bob.age * members.alice[1]
