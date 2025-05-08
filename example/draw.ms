type Color = ( white | black | grey | blue | violet | green | red | pink | yellow );

load init_canvas(): void;
load draw(x: int, y: int, color: Color): void;

init_canvas();

let top = 10;
let left = 10;
let height = 200;
let width = 300;

let y_index = top;
while y_index < (height + top) loop {
    let x_index = left;
    while x_index < (width + left) loop {
        let one_third = width / 3;
        let color = {
            if  one_third > (x_index - left)
                then Color::blue
            else if one_third * 2 >  (x_index - left)
                then Color::white
                else Color::red
        };
        draw(x_index, y_index, color);
        let x_index + 1
    };
    let y_index + 1
};
"France flag"
