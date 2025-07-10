~~ Function around string ~~
load concat(a: str, b: str): str;
load split(a: str, b: str): [str];
load strcmp(a: str, b: str):bool;
~~ Function around array ~~
load slice(a: [str], b: int, c: int): [str];
load append(a: [str], b: [str]): [str];
load join(a: [str], b: str): str;
load arrlen(a: [str]): int;
~~ Function around type cast ~~
load to_str(a: num): str;
load to_num(a: str): int;

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

let lexer(source: str) = {
    let chars = source.split("");
    let length = arrlen(chars);

    let current = "";
    let tokens = [str]!;
    let nest_level = 0;
    let index = 0;

    while index < length loop {
        let char = chars[index];
        if char == "(" then {
            let nest_level + 1;
            let current + char
        } else if char == ")" then {
            let nest_level - 1;
            let current + char
        } else if (char == " ") && (nest_level == 0) then {
            let current = "";
            let new = [current].memcpy();
            if tokens? then {
                tokens.append(new)
            } else {
                tokens = [new]
            }
        } else {
            let current + char
        };
        index += 1;
    }
    tokens
};

let parse(source: str): Expr = {
    let tokens = lexer(source);
    if arrlen(tokens) == 1 then node_literal_int(tokens[0]: int)
    else if tokens[-2] == "+" then {
        let lhs = tokens.slice(0, -2).join(" ").parse();
        node_binop_add(lhs, parse(tokens[-1]))
    }
    else Expr!
};

let ast = parse("100 + 200 + 3");
ast.compile()
