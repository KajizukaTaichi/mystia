type T = int;
type Status = ( Some | None );
type Option = @{ status: Status, value: T };

pub let Some(value: T) = memcpy(@{ status: Status#Some, value: value });
pub let None = memcpy(@{ status: Status#None, value: null as T });

let unwrap(value: Option) = value.value;
let unwrap_or(value: Option, or: T) = {
    if value.status == Status#Some
        then value.value
        else or
};

let a = Some(1);
let b = None;

unwrap_or(b, unwrap(a))
