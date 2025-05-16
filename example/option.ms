type T = [int; 3];
type Status = ( Some | None );
type Option = @{ value: T, status: Status };

pub let Some(value: T) = memcpy(@{ value: value, status: Status#Some });
pub let None = memcpy(@{ value: null as T, status: Status#None });

let unwrap(value: Option) = value.value;
let unwrap_or(value: Option, or: T) = {
    if value.status == Status#Some
        then value.value
        else or
};

let a = Some([1,2,3]);
let b = None;

unwrap_or(b, unwrap(a))
