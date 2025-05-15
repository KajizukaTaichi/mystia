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
        panel: 0
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

let app() = {
    let frame = new_elm("div", 0);
    upd_elm(frame, "style", "margin: 100px");
    evt_elm(frame, "keydown", "inc_key");

    let title = new_elm("h1", frame);
    upd_elm(title, "innerHTML", model.title);

    let model.layout.panel = new_elm("p", frame);
    upd_elm(model.layout.panel, "style", "font-family: monospace");
    view();

    let btn = new_elm("button", frame);
    upd_elm(btn, "innerHTML", "Increment");
    upd_elm(btn, "className", "button");
    evt_elm(btn, "click", "inc_btn")
};

app();
model.title
