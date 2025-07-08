type LinkList = @{ value: int, next: LinkList };

let car(self: LinkList) = self.value;
let cdr(self: LinkList) = self.next;
let append(self: LinkList, other: int) = self.next = @{ value: other, next: null: LinkList };

let list = @{
    value: 2,
    next: @{
        value: 5,
        next: null: LinkList
    }
};

list.car() + list.cdr().car()
