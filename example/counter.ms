load new_elm(id: str, tag: str): void;
load set_elm(id: str, content: str): str;
load tap_elm(id: str, funcname: str): str;
load get_status(id: str): int;
load set_status(id: str, val: int): void;

let inc() = {
    let current = get_status("number");
    let next = current + 1;
    set_status("number", next);
    set_elm("view", next);
};

new_elm("view", "h1");
new_elm("btn", "button");

tap_elm("btn", "inc")
