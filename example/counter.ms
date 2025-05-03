load write(_: str): void;
load concat(_1: str, _2: str): str;
load int_to_str(_: int): str;

let a = int_to_str(1+2);
write(concat("<h1>Count", concat(a, "</h1>")));
