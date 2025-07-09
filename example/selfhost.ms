load concat(a: str, b: str): str;
load to_str(a: int): str;
load to_num(a: str): int;
load print(a: str): void;

type Kind = ( Add | Literal );
type Expr = @{ kind: Kind, literal_int: int, lhs: Expr, rhs: Expr };

let compile(expr: Expr): str = {
    if expr.kind == Kind#Literal then
        "(i32.const " + to_str(expr.literal_int) + ")"
    else if (expr.kind == Kind#Add) && (expr.lhs?) && (expr.rhs?) then {
        let lhs = expr.lhs.compile();
        let rhs = expr.rhs.compile();
        "(i32.add " + lhs + " " + rhs + ")"
    }
    else str!
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

let ast = node_binop_add(
    node_literal_int(200),
    node_literal_int(200)
);
ast
