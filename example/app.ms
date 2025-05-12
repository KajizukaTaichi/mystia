type Model = @{
    number: int,
};

load get_status(id: str): Model;
load set_status(id: str, val: Model): void;
load new_elm(id: str, tag: str): void;
load set_elm(id: str, content: str): void;
load tap_elm(id: str, func: str): void;
load int_to_str(n: int): str;
load concat(a: str, b: str): str;

let inc() = {
    let model = get_status("model");
    let model.number + 1;
    let num = int_to_str(model.number);
    set_elm("view", concat("Number: ", num))
};

let app_name = "Counter App";
set_status("number", @{ number: 0 });

new_elm("title", "h1");
set_elm("title", app_name);

new_elm("view", "p");
set_elm("view", "Number: 0");

new_elm("btn", "button");
set_elm("btn", "Increment");
tap_elm("btn", "inc");

app_name
