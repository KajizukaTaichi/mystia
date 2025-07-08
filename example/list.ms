type LinkList = @{ value: int, next: LinkList };

let car(n: LinkList) = n.value;
let cdr(n: LinkList) = n.next;

let list = @{
    value: 2,
    next: @{
        value: 5,
        next: null: LinkList
    }
};

list
