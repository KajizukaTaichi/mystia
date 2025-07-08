type LinkList = @{ value: int, next: LinkList };

let append(self: LinkList, new: int) = { while self.next != (null:LinkList) loop { let self = self.next } let self.next = @{ value: new, next: null:LinkList } };

let list = @{ value: 2, next: @{ value: 5, next: @{ value: 8, next: null:LinkList } } };

list.car() + list.cdr().car() + list.cdr().cdr().car()
