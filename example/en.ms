type Color = ( white | black | grey | blue | violet | green | red | pink | yellow );

load init_canvas(): void;
load draw(x: int, y: int, color: Color): void;

load pi(): num;
load sin(_: num): num;
load cos(_: num): num;
load rad(_: int): num;

let draw_circle(cx: num, cy: num, r: num) = {
    let angle = 0;
    while angle < 360 loop {
        let rad = rad(angle);
        let x = cx + r * cos(rad);
        let y = cy + r * sin(rad);
        draw(x: int, y: int, Color#red);
        draw(x: int + 1, y: int, Color#red);
        draw(x: int + 2, y: int, Color#red);
        let angle + 1
    }
};

init_canvas();
draw_circle(100.0, 50.0, 50.0);
0
