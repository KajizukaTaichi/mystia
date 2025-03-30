let members = dict{
    let alice = 13;
    let bob = true
};

let members.alice = 100;
if members.bob
    then members.alice
    else 50
