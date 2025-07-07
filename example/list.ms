type LinkList = @{ value: int, next: LinkList };

let list = @{
    value: 2,
    next: @{
        value: 5,
        next: null: LinkList
    }
};
list.next.value
