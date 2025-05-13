type Counter = @{
    number: int,
};

load model(): Counter;
load init_model(val: Counter): void;
load new_elm(id: str, tag: str): void;
load upd_elm(id: str, prop: str, content: str): void;
load evt_elm(id: str, name: str, func: str): void;
load int_to_str(n: int): str;
load concat(a: str, b: str): str;

let inc() = {
    let model().number + 1;
    let num = int_to_str(model().number);
    upd_elm("view", "innerHTML", concat("Number: ", num))
};

let app_name = "Counter App";
init_model(@{ number: 0 });

new_elm("title", "h1");
upd_elm("title", "innerHTML", "Counter App");

new_elm("view", "p");
upd_elm("view", "style", "font-family: monospace");
upd_elm("view", "innerHTML", "Number: 0");

new_elm("btn", "button");
upd_elm("btn", "innerHTML", "Increment");
evt_elm("btn", "click", "inc");

app_name
