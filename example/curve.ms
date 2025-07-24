type Color = @{ r: int, g: int, b: int };

load init_canvas(): void;
load draw(x: int, y: int, color: Color): void;

load math.pi(): num;
load math.sin(value: num): num;
load math.cos(value: num): num;
load math.rad(value: int): num;

pub let color = @{ r: 255, g: 100, b: 100 };
let draw_circle(cx: num, cy: num, r: num) = {
    let angle = 0;
    while angle < 360 loop {
        let rad = rad(angle);
        let x = cx + r * cos(rad);
        let y = cy + r * sin(rad);

        let i = 0;
        while i < 100 loop {
            draw(x: int + i, y: int + i, color);
            let i + 1
        };
        let angle + 1
    }
};

init_canvas();
draw_circle(100.0, 50.0, 50.0)
