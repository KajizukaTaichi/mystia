type LinkList = @{ value: int, next: LinkList };

let node(value: int) = memcpy(@{ value: value, next: LinkList! });
let append(self: LinkList, other: LinkList) = {
    while self.next? loop {
        let self = self.next
    };
    let self.next = other;
    self
};

let a = node(1);
let b = node(2).append(node(3));
a.append(b);

a.value + a.next.value + a.next.next.value
