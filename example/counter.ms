load new_elm(id: str, tag: str): void;
load set_elm(id: str, content: str): void;
load tap_elm(id: str, funcname: str): void;
load get_status(id: str): int;
load set_status(id: str, val: int): void;
load int_to_str(n: int): str;
load concat(a: str, b: str): str;

let inc() = {
    let current = get_status("number");
    let current + 1;
    set_status("number", current);
    set_elm("view", concat("Number: ", int_to_str(current)))
};

set_status("number", 0);

new_elm("title", "h1");
set_elm("title", "Counter App");

new_elm("view", "p");
sew_elm("view", "Number: 0");

new_elm("btn", "button");
set_elm("btn", "Increment");
tap_elm("btn", "inc");

"Counter App"
