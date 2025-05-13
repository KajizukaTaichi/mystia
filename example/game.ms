load new_elm(id: str, tag: str, parent: str): void;
load upd_elm(id: str, prop: str, content: str): void;
load evt_elm(id: str, name: str, func: str): void;
load int_to_str(n: int): str;
load concat(a: str, b: str): str;
load alert(msg: str): void;

pub let model = @{
    lives: 5,
    player: @{
        id: "player",
        position: @{ x: 1, y: 0 }
    }
};

let x_position() = concat("left: ", concat(int_to_str(model.player.position.x), "px;"));
let y_position() = concat("bottom: ", concat(int_to_str(model.player.position.y), "px;"));
let draw_player() = {
    upd_elm(model.player.id, "style", x_position());
    upd_elm(model.player.id, "style", y_position())
};

let move_player(n: int) = {
    let model.player.position.x + n;
    draw_player()
};

pub let user_operation(key_code: int) = {
    if key_code == 39 then {
        move_player(100)
    } else if key_code == 37 then {
        move_player(-100)
    }
};

let game() = {
    new_elm(model.player.id, "img", "body");
    upd_elm(model.player.id, "src", "https://pixiv.navirank.com/jpg/153/15371090.jpg");
    upd_elm(model.player.id, "style", "height: 100px; position: absolute;");
    evt_elm(model.player.id, "keydown", "user_operation")
};

game(); 0
