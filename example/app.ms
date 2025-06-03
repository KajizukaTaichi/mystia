type Elm = int;

load new_elm(tag: str, parent: Elm): Elm;
load upd_elm(id: Elm, prop: str, content: str): void;
load evt_elm(id: Elm, name: str, func: str): void;
load concat(a: str, b: str): str;
load to_str(n: num): str;

pub let model = @{
    title: "Counter App",
    count: 0,
    layout: @{
        panel: null:Elm
    }
};

let view() = {
    let formatted = "Number: " + (model.count as str);
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

let frame = new_elm("div", null: Elm);
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
