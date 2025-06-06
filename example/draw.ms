~~ Create by ChocolateKun ~~

type Color = ( white | black | grey | blue | violet | green | red | pink | yellow );
let colors = [Color#red, Color#black, Color#blue, Color#violet, Color#green, Color#white, Color#yellow];
let colors_num = 7;

load init_canvas(): void;
load draw(x: int, y: int, color: Color): void;

init_canvas();

let top = 10;
let left = 10;
let height = 600;
let width = 900;

let ysiz = height;
let xsiz = width / 2;
let yidx = top;
let xidx = left;

let state = 0;
let color_index = 0;
while (xsiz >= 2) || (ysiz >= 2) loop {
    let y_index = yidx;
    while y_index < (yidx + ysiz) loop {
        let x_index = xidx;
        while x_index < (xidx+xsiz) loop {
            draw(x_index, y_index, colors[color_index % colors_num]);
            let x_index + 1
        };
        let y_index + 1
    };
    let color_index = color_index + 1;

    if state == 0 then {
        let xidx + xsiz;
        let ysiz / 2
    } else if state == 1 then {
        let yidx + ysiz;
        let xsiz / 2;
        let xidx + xsiz
    } else if state == 2 then {
        let xidx - xsiz;
        let ysiz / 2;
        let yidx + ysiz
    } else {
        let yidx - ysiz;
        let xsiz / 2
    };
    let state + 1;
    let state % 4
};
"zukei"
