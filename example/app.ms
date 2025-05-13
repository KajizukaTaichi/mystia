load new_elm(id: str, tag: str, parent: str): void;
load upd_elm(id: str, prop: str, content: str): void;
load evt_elm(id: str, name: str, func: str): void;
load int_to_str(n: int): str;
load concat(a: str, b: str): str;

type Counter = @{
    number: int,
};
pub let model = @{ number: 0 };

pub let inc() = {
    let model.number + 1;
    let num = int_to_str(model.number);
    upd_elm("view", "innerHTML", concat("Number: ", num))
};

new_elm("container", "div", "body");
upd_elm("container", "style", "margin: 100px");

new_elm("title", "h1", "container");
upd_elm("title", "innerHTML", "Counter App");

new_elm("view", "p", "container");
upd_elm("view", "style", "font-family: monospace");
upd_elm("view", "innerHTML", "Number: 0");

new_elm("btn", "button", "container");
upd_elm("btn", "innerHTML", "Increment");
upd_elm("btn", "className", "button");
evt_elm("btn", "click", "inc");
0
