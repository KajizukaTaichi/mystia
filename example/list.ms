type LinkList = @{ value: int, next: LinkList };
load print(_: str): void;
load to_str(_: int): str;

let car(self: LinkList) = self.value;
let cdr(self: LinkList) = self.next;
let node(value: int) = memcpy(@{ value: value, next: LinkList! });
let append(self: LinkList, other: LinkList) = {
    let original_object = self;
    while self.next? loop { let self = self.next };
    let self.next = other;
    original_object
};

let a = node(1);
let b = node(2).append(node(3));
let c = b.append(a);
c
