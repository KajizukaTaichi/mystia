load new_elm(id: str, tag: str, parent: str): void;
load upd_elm(id: str, prop: str, content: str): void;
load evt_elm(id: str, name: str, func: str): void;
load to_str(n: num): str;
load concat(a: str, b: str): str;
load alert(msg: str): void;
load rand(): num;

type Character = @{
    id: str,
    src: str,
    position: @{ x: int, y: int }
};

pub let model = @{
    player: @{
        id: "player",
        src: "https://pixiv.navirank.com/jpg/153/15371090.jpg",
        position: @{ x: 1, y: 0 }
    },
    target: @{
        id: "target",
        src: "https://pixiv.navirank.com/jpg/1263/126387033.jpg",
        position: @{ x: 500, y: 500 }
    },
    style: @{
        char: "height: 100px; position: absolute;"
    },
};

let x_position(n: Character) = "left: " + (n.position.x:num as str) + "px;";
let y_position(n: Character) = "bottom: " + (n.position.y:num as str) + "px;";
let draw_player() = {
    upd_elm(model.player.id, "style", x_position(model.player));
    upd_elm(model.player.id, "style", y_position(model.player))
};
let draw_target() = {
    upd_elm(model.target.id, "style", x_position(model.target));
    upd_elm(model.target.id, "style", y_position(model.target))
};

let is_touch() = {
    let target_x = model.target.position.x / 100;
    let target_y = model.target.position.y / 100;
    let player_x = model.player.position.x / 100;
    let player_y = model.player.position.y / 100;
    (player_x == target_x) && (player_y == target_y)
};

let move_target() = {
    let model.target.position.x + (rand() * 100.0: int - 40);
    let model.target.position.y + (rand() * 100.0: int - 40);
    draw_target()
};
let move_player(x_move: int, y_move: int) = {
    let model.player.position.x + x_move;
    let model.player.position.y + y_move;
    draw_player(); move_target();
    if is_touch() then alert("Touch!")
};

pub let user_operation(key_code: int) = {
    if key_code == 39 then  move_player(100, 0)
    else if key_code == 37 then move_player(-100, 0)
    else if key_code == 38 then move_player(0, 100)
    else if key_code == 40 then move_player(0, -100)
};

let game() = {
    new_elm(model.player.id, "img", "body");
    upd_elm(model.player.id, "src", model.player.src);
    upd_elm(model.player.id, "style", model.style.char);
    evt_elm(model.player.id, "keydown", "user_operation");

    new_elm(model.target.id, "img", "body");
    upd_elm(model.target.id, "src", model.target.src);
    upd_elm(model.target.id, "style", model.style.char);

    draw_player(); draw_target()
};

game();
"Nazrin Toucher"
