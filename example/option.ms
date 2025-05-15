type T = [int; 3];
type Status = ( Some | None );
type Option = @{ status: Status, value: T };

let Some(value: T) = @{ status: Status::Some, value: value };
let None = @{ status: Status::None, value: null as T };

let unwrap(value: Option) = value.value;
let unwrap_or(value: Option, or: T) = {
    if null: T != value.value
        then value.value
        else or
};

let a = Some([1, 2, 3]);
let b = None;

unwrap_or(b, unwrap(a))
