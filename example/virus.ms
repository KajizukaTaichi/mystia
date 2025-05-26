load os::{ write_file(path: str, content: str): void };
load repeat(str: str, count: int): str;
load concat(a: str, b: str): str;
load to_str(a: int): str;

let i = 0;
while i < 100 loop {
    let path = i.to_str() + ".txt";
    let content = "Hacked! ".repeat(100);
    write_file(path, content);
    let i + 1
}
