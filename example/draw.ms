type Color = ( white | black | grey | blue | violet | green | red | pink | yellow );

load init_canvas(): void;
load draw(x: int, y: int, color: Color): void;

init_canvas();

let top = 100;
let left = 100;
let height = 10;
let width = 15;

let index = top;
while index < (height + top) loop {
    let jndex = left;
    while jndex < (width + left) loop {
        let color = {
            if jndex < (width / 3)
                then Color::blue
            else if jndex < (width / 3 * 2)
                then Color::white
                else Color::red
        };
        draw(jndex, index, color);
        let jndex + 1
    };
    let index + 1
};
0
