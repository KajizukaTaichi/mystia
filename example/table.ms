load print(s: str): void;
load concat(s: str, v: str): str;
load to_str(s: num): str;

let i = 1;
while i < 10 loop {
    let j = 1;
    while j < 10 loop {
        print(f"{i} x {j} = {i * j}");
        let j + 1
    };
    let i + 1
}
