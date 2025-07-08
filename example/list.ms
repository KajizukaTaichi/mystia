type LinkList = @{ value: int, next: LinkList };

let append(self: LinkList, new: int) = {
    while self.next? loop {
        let self = self.next
    };
    let self.next = @{ value: new, next: LinkList! }
};

let list = @{ value: 2, next: @{ value: 5, next: @{ value: 8, next: LinkList! } } }
