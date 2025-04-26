type Color = ( white | black | grey | blue | violet | green | red | pink | yellow );

load init_canvas(): void;
load draw(x: int, y: int, color: Color): void;

init_canvas();

let height = 10;
let width = 15;
let index = 0;

while index < height loop {
    let jndex = 0;
    while jndex < width loop {
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
