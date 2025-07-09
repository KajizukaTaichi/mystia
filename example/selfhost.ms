load concat(a: str, b: str): str;
load to_str(a: int): str;
load to_num(a: str): int;

type Kind = ( Add | Literal );
type Expr = @{ kind: Kind, literal_int: int, term: [Expr] };

let compile(expr: Expr): str = {
    if expr.kind == Kind#Literal then "(i32.const " + to_str(expr.literal_int) + ")"
    else "(i32.add " + compile(expr.term[0]) + " " + compile(expr.term[1]) + ")"
};

let node_literal_int(value: int) = memcpy(@{
    kind: Kind#Literal,
    literal_int: value,
    term: [Expr]!
});

let node_binop_add(a: Expr, b: Expr) = memcpy(@{
    kind: Kind#Add,
    literal_int: int!,
    term: [a, b]
});

let ast = node_literal_int(42)
