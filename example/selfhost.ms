load concat(a: str, b: str): str;
load split(a: str, b: str): [str];
load strcmp(a: str, b: str): bool;
load strlen(a: str): int;
load slice(a: [str], b: int, c: int): [str];
load append(a: [str], b: [str]): [str];
load join(a: [str], b: str): str;
load arrlen(a: [str]): int;
load to_str(a: num): str;
load to_num(a: str): int;

type Kind = ( Add | Sub | Literal );
type Expr = @{ kind: Kind, literal_int: int, lhs: Expr, rhs: Expr };

let compile(expr: Expr): str = {
    macro binop(opname) = "(i32." + opname + " "
        + expr.lhs.compile() + " "
        + expr.rhs.compile() + ")";
    if expr.kind == Kind#Literal then
        "(i32.const " + (expr.literal_int: str) + ")"
    else if expr.kind == Kind#Add then binop("add")
    else if expr.kind == Kind#Sub then binop("sub")
    else str!
};

let node_literal_int(value: int) = memcpy(@{
    kind: Kind#Literal,
    literal_int: value,
    lhs: Expr!, rhs: Expr!
});

let node_binop(k: Kind, a: Expr, b: Expr) = memcpy(@{
    kind: k,
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

    macro add_token() = {
        let tokens = {
            if tokens? then {
                tokens.append([current].memcpy())
            } else {
                [current].memcpy()
            }
        }
    };

    while index < length loop {
        let char = chars[index];
        if char == "(" then {
            let nest_level + 1;
            let current + char
        } else if char == ")" then {
            let nest_level - 1;
            let current + char
        } else if (char == " ") && (nest_level == 0) then {
            add_token();
            let current = ""
        } else {
            let current + char
        };
        let index + 1
    };
    if strlen(current) != 0 then add_token();
    tokens
};

let parse(source: str): Expr = {
    let tokens = lexer(source);
    if arrlen(tokens) == 1 then {
        let chars = tokens[0].split("");
        macro bracket_inner(chars) = chars.slice(1, -1).join("");
        if (chars[0] == "(") && (chars[-1] == ")") then {
            parse(bracket_inner(chars))
        } else {
            node_literal_int(tokens[0]: int)
        }
    } else {
        macro is_op(opname) = tokens[-2] == opname;
        macro binop(opkind) = opkind.node_binop(
            parse(tokens.slice(0, -2).join(" ")),
            parse(tokens[-1])
        );
        if is_op("+") then { return binop(Kind#Add) };
        if is_op("-") then { return binop(Kind#Sub) };
        Expr!
    }
};

let ast = parse("1 + (2 - 3)");
ast.compile()
