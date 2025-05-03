load write(_: str): void;
load concat(_: str, _: str): str;
load int_to_str(_: int): str;

let a = int_to_str(1+2);
write(concat("Count", a));
;
0
