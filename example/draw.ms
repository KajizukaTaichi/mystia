type Color = ( red | blue | green | yellow );

load init_canvas(): void;
load draw(x: int, y: int, color: Color): void;

init_canvas();
draw(0, 0, Color::red);
