type Color = ( white | black | grey | blue | violet | green | red | pink | yellow );

load init_canvas(): void;
load draw(x: int, y: int, color: Color): void;

init_canvas();

let height = 10;
let width = 15;

let y_index = 0;
while y_index < height loop {
    let x_index = 0;
    while x_index < width loop {
        let one_third = width / 3;
        let color = {
            if  one_third > x_index
                then Color::blue
            else if one_third * 2 > x_index
                then Color::white
                else Color::red
        };
        draw(x_index, y_index, color);
        let x_index + 1
    };
    let y_index + 1
};
0
