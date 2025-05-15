load new_elm(id: str, tag: str, parent: str): void;
load upd_elm(id: str, prop: str, content: str): void;
load evt_elm(id: str, name: str, func: str): void;
load concat(a: str, b: str): str;
load to_str(n: num): str;

pub let model = @{
    count: 0,
    title: "Counter App",
    layout: @{
        frame: "container",
        title: "app-title",
        view: "number-view",
        btn: "inc-button"
    },
    handlers: @{
        btn: "inc_btn",
        key: "inc_key"
    }
};

let format() = {
    let view = "Number: " + (model.count: num as str);
    upd_elm(model.layout.view, "innerHTML", view)
};

pub let inc_btn() = {
    let model.count + 1;
    format()
};

pub let inc_key(key_code: int) = {
    if key_code == 39 then {
        let model.count + 1
    } else if key_code == 37 then {
        let model.count - 1
    };
    format()
};

let app() = {
    new_elm(model.layout.frame, "div", "body");
    upd_elm(model.layout.frame, "style", "margin: 100px");
    evt_elm(model.layout.frame, "keydown", model.handlers.key);

    new_elm(model.layout.title, "h1", model.layout.frame);
    upd_elm(model.layout.title, "innerHTML", model.title);

    new_elm(model.layout.view, "p", model.layout.frame);
    upd_elm(model.layout.view, "style", "font-family: monospace");
    format();

    new_elm(model.layout.btn, "button", model.layout.frame);
    upd_elm(model.layout.btn, "innerHTML", "Increment");
    upd_elm(model.layout.btn, "className", "button");
    evt_elm(model.layout.btn, "click", model.handlers.btn)
};

app();
model.title
