load new_elm(id: str, tag: str): void;
load set_elm(id: str, content: str): void;
load tap_elm(id: str, funcname: str): void;
load get_status(id: str): int;
load set_status(id: str, val: int): void;
load int_to_str(n: int): str;

let incd() = {
    let current = get_status("number");
    let current + 1;
    set_status("number", current);
    set_elm("view", int_to_str(current))
};

new_elm("view", "h1");
new_elm("btn", "button");

tap_elm("btn", "incd");
0
