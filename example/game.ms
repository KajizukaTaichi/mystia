type Elm = int;

load new_elm(tag: str, parent: Elm): Elm;
load upd_elm(id: Elm, prop: str, content: str): void;
load evt_elm(id: Elm, name: str, func: str): void;
load random::{ randint(a: int, b: int): int };
load to_str(n: num): str;
load concat(a: str, b: str): str;
load alert(msg: str): void;

type Character = @{
    dom: int, src: str,
    position: @{ x: int, y: int }
};

pub let model = @{
    count: 0,
    speed: 50,
    player: @{
        dom: null:Elm,
        src: "https://pixiv.navirank.com/jpg/153/15371090.jpg",
        position: @{ x: null:int, y: null:int }
    },
    target: @{
        dom: null:Elm,
        src: "https://pixiv.navirank.com/jpg/1263/126387033.jpg",
        position: @{ x: null:int, y: null:int }
    },
    style: @{
        char: "height: 100px; position: absolute;"
    },
};

let init_position() = {
    let model.player.position.x = 0;
    let model.player.position.y = 0;
    let model.target.position.x = 500;
    let model.target.position.y = 500
};

let x_position(n: Character) = "left: " + (n.position.x as str) + "px;";
let y_position(n: Character) = "bottom: " + (n.position.y as str) + "px;";
let draw_player() = {
    model.player.dom.upd_elm("style", x_position(model.player));
    model.player.dom.upd_elm("style", y_position(model.player))
};
let draw_target() = {
    model.target.dom.upd_elm("style", x_position(model.target));
    model.target.dom.upd_elm("style", y_position(model.target))
};

let is_touch() = {
    let target_x = model.target.position.x / 100;
    let target_y = model.target.position.y / 100;
    let player_x = model.player.position.x / 100;
    let player_y = model.player.position.y / 100;
    (player_x == target_x) && (player_y == target_y)
};

let move_target() = {
    let model.target.position.x + randint(-model.speed, model.speed);
    let model.target.position.y + randint(-model.speed, model.speed);
    draw_target()
};
let move_player(x_move: int, y_move: int) = {
    let model.player.position.x + x_move;
    let model.player.position.y + y_move;
    draw_player(); move_target();
    if is_touch() then {
        let model.count + 1;
        alert("Touch! " + (model.count: str) + "th times");
        init_position(); draw_player(); move_target()
    }
};

pub let user_operation(key_code: int) = {
    if key_code == 39 then  move_player(model.speed, 0)
    else if key_code == 37 then move_player(-model.speed, 0)
    else if key_code == 38 then move_player(0, model.speed)
    else if key_code == 40 then move_player(0, -model.speed)
};

let model.player.dom = new_elm("img", null:Elm);
model.player.dom.upd_elm("src", model.player.src);
model.player.dom.upd_elm("style", model.style.char);
model.player.dom.evt_elm("keydown", "user_operation");

let model.target.dom = new_elm("img", null:Elm);
model.target.dom.upd_elm("src", model.target.src);
model.target.dom.upd_elm("style", model.style.char);

init_position();
draw_player();
draw_target();
"Nazrin Toucher"
