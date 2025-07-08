type LinkList = @{ value: int, next: LinkList };

let car(self: LinkList) = self.value;
let cdr(self: LinkList) = self.next;

let list = @{ value: 2, next: @{ value: 5, next: @{ value: 8, next: null:LinkList } } };

list.car() + list.cdr().car() + list.cdr().cdr().car()
