load concat(a: str, b: str): str;
load slice(a: [str], b: int, c: int): [str];
load strcmp(a: str, b: str):bool;
load split(a: str, b: str): [str];
load join(a: [str], b: str): str;
load arrlen(a: [str]): int;
load to_str(a: num): str;
load to_num(a: str): int;
load print(a: str): void;

type Kind = ( Add | Literal );
type Expr = @{ kind: Kind, literal_int: int, lhs: Expr, rhs: Expr };

let compile(expr: Expr): str = {
    if expr.kind == Kind#Literal then
        "(i32.const " + (expr.literal_int: str) + ")"
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

let parse(source: str): Expr = {
    let tokens = source.split(" ");
    if arrlen(tokens) == 1 then node_literal_int(tokens[0]: int)
    else if tokens[-2] == "+" then {
        let lhs = tokens.slice(0, -2).join(" ");
        node_binop_add(lhs.parse(), parse(tokens[-1]))
    }
    else Expr!
};

let ast = parse("100 + 200 + 3");
ast.compile()
