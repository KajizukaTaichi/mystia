type LinkList = @{ value: int, relation: LinkList };

pub let node(value: int) = memcpy(@{ value: value, relation: LinkList! });
pub let append(self: LinkList, other: LinkList) = {
    let current = self;
    while current.relation? loop {
        let current = current.relation
    };
    let current.relation = other;
    self
};

let a = node(1);
let b = node(2).append(node(3));
b.append(a)
