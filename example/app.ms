type Model = @{
    number: int,
};

load get_status(): Model;
load set_status(val: Model): void;
load new_elm(id: str, tag: str): void;
load set_elm(id: str, prop: str, content: str): void;
load tap_elm(id: str, func: str): void;
load int_to_str(n: int): str;
load concat(a: str, b: str): str;

let inc() = {
    let get_status().number + 1;
    let num = int_to_str(get_status().number);
    set_elm("view", "innerHTML", concat("Number: ", num))
};

let app_name = "Counter App";
set_status(@{ number: 0 });

new_elm("title", "h1");
set_elm("title", "innerHTML", "Number: 0");

new_elm("view", "p");
set_elm("view", "innerHTML", "Number: 0");

new_elm("btn", "button");
set_elm("btn", "innerHTML", "Increment");
tap_elm("btn", "inc");

app_name
