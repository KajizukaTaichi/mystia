type LinkList = @{ value: int, next: LinkList };

let node(value: int) = memcpy(@{ value: value, next: LinkList! });
let append(self: LinkList, other: LinkList) = {
    while self.next? loop {
        let self = self.next
    };
    let self.next = other
};

let list = @{ value: 2, next: @{ value: 5, next: @{ value: 8, next: LinkList! } } }
