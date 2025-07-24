type LinkList = @{ value: int, relate: LinkList };

pub let node(value: int) = memcpy(@{ value: value, relate: LinkList! });
pub let append(self: LinkList, other: LinkList) = {
    let current = self;
    while current.relate? loop {
        let current = current.relate
    };
    let current.relate = other;
    self
};

let a = node(1);
let b = node(2).append(node(3));
b.append(a)
