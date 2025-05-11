type point = @{ x: int, y: int, z: int };
let sum_point(p: point) = p.x + p.y + p.z;

let foo = [100, 200, 300];
let bar = foo as point;
sum_point(bar)
