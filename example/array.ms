type point = { x: int, y: int, z: int };
let foo = [100, 200, 300];
let bar = foo: point;
bar.x + bar.y + bar.z
