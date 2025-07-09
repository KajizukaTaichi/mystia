load concat(a: str, b: str): str;
load to_str(a: int): str;
load to_num(a: str): int;

type Kind = ( Add | Literal );
type Expr = @{ kind: Kind, literal_int: int, lhs: Expr, rhs: Expr };

let compile(expr: Expr): str = {
    if expr.kind == Kind#Literal then "(i32.const " + to_str(expr.literal_int) + ")"
    else "(i32.add " + compile(expr.lhs) + " " + compile(expr.rhs) + ")"
};

let node_literal_int(value: int) = memcpy(@{
    kind: Kind#Literal,
    literal_int: value,
    lhs: Expr!, rhs: Expr!
});

let node_binop_add(a: Expr, b: Expr) = memcpy(@{
    kind: Kind#Add,
    literal_int: int!,
    lhs: a, rhs: b
});

node_literal_int(200)
