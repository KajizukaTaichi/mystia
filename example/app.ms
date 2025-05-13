type Model = @{
    number: int,
};

load get_status(): Model;
load upd_status(val: Model): void;
load new_elm(id: str, tag: str): void;
load upd_elm(id: str, prop: str, content: str): void;
load evt_elm(id: str, name: str, func: str): void;
load int_to_str(n: int): str;
load concat(a: str, b: str): str;

let inc() = {
    let get_status().number + 1;
    let num = int_to_str(get_status().number);
    upd_elm("view", "innerHTML", concat("Number: ", num))
};

let app_name = "Counter App";
upd_status(@{ number: 0 });

new_elm("title", "h1");
upd_elm("title", "innerHTML", "Number: 0");

new_elm("view", "p");
upd_elm("view", "innerHTML", "Number: 0");

new_elm("btn", "button");
upd_elm("btn", "innerHTML", "Increment");
tap_elm("btn", "inc");

app_name
