type LinkList = @{ value: int, next: LinkList };

pub let node(value: int) = memcpy(@{ value: value, next: LinkList! });
pub let append(self: LinkList, other: LinkList) = {
    let current = self;
    while current.next? loop {
        let current = current.next
    };
    let current.next = other;
    self
};

let a = node(1);
let b = node(2).append(node(3));
b.append(a)
