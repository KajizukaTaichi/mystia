load new_elm(tag: str, parent: int): int;
load upd_elm(id: int, prop: str, content: str): void;
load evt_elm(id: int, name: str, func: str): void;
load concat(a: str, b: str): str;
load to_str(n: num): str;

pub let model = @{
    title: "Counter App",
    count: 0,
    layout: @{
        panel: -1
    }
};

let view() = {
    let formatted = f"Number: {model.count: str}";
    upd_elm(model.layout.panel, "innerHTML", formatted)
};

pub let inc_btn() = {
    let model.count + 1;
    view()
};

pub let inc_key(key_code: int) = {
    if key_code == 39 then {
        let model.count + 1
    } else if key_code == 37 then {
        let model.count - 1
    };
    view()
};

let frame = new_elm("div", -1);
frame.upd_elm("style", "margin: 100px");
frame.evt_elm("keydown", "inc_key");

let title = new_elm("h1", frame);
title.upd_elm("innerHTML", model.title);

let panel = new_elm("p", frame);
let model.layout.panel = panel;
panel.upd_elm("style", "font-family: monospace");
view();

let btn = new_elm("button", frame);
btn.upd_elm("innerHTML", "Increment");
btn.upd_elm("className", "button");
btn.evt_elm("click", "inc_btn");
model.title
