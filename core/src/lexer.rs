use crate::*;

pub fn tokenize(
    input: &str,
    delimiter: &[&str],
    is_expr: bool,
    is_trim: bool,
    is_split: bool,
) -> Option<Vec<String>> {
    let mut tokens: Vec<String> = Vec::new();
    let mut current_token = String::new();
    let mut in_parentheses: usize = 0;
    let mut in_quote = false;
    let mut is_comment = false;
    let mut is_escape = false;

    let chars: Vec<String> = input.chars().map(String::from).collect();
    let mut index = 0;

    while index < chars.len() {
        let c = chars.get(index)?.to_owned();
        if include_letter("~~", &chars, index) && !in_quote {
            is_comment = !is_comment;
            index += 2;
            continue;
        }
        if is_comment {
            index += 1;
            continue;
        }
        if is_escape {
            current_token.push_str(match c.as_str() {
                "n" => "\n",
                "t" => "\t",
                "r" => "\r",
                _ => &c,
            });
            is_escape = false;
            index += 1;
        } else if ["(", "[", "{"].contains(&c.as_str()) {
            if is_split && in_parentheses == 0 {
                tokens.push(current_token.clone());
                current_token.clear();
            }
            current_token.push_str(c.as_str());
            in_parentheses += 1;
            index += 1;
        } else if [")", "]", "}"].contains(&c.as_str()) {
            current_token.push_str(c.as_str());
            if let Some(i) = in_parentheses.checked_sub(1) {
                in_parentheses = i;
            } else {
                return None;
            }
            index += 1;
        } else if ["\"", "'", "`"].contains(&c.as_str()) {
            in_quote = !in_quote;
            current_token.push_str(c.as_str());
            index += 1;
        } else if c == "\\" {
            current_token.push_str(&c);
            is_escape = true;
            index += 1;
        } else {
            let mut is_opr = false;
            if is_expr {
                for op in OPERATOR {
                    if include_letter(op, &chars, index) && in_parentheses == 0 && !in_quote {
                        if current_token.is_empty() {
                            index += op.chars().count();
                            tokens.push(op.to_string());
                        } else {
                            tokens.push(current_token.to_string());
                            index += op.chars().count();
                            tokens.push(op.to_string());
                            current_token.clear();
                        }
                        is_opr = true;
                        break;
                    }
                }
            }
            let mut is_delimit = false;
            if !is_opr {
                for delimit in delimiter {
                    if include_letter(delimit, &chars, index) && in_parentheses == 0 && !in_quote {
                        if current_token.is_empty() {
                            index += delimit.chars().count();
                        } else {
                            tokens.push(current_token.clone());
                            index += delimit.chars().count();
                            current_token.clear();
                        }
                        is_delimit = true;
                        break;
                    }
                }
                if !is_delimit {
                    current_token.push_str(c.as_str());
                    index += 1;
                }
            }
        }
    }

    // Syntax error check
    if is_escape || in_quote || in_parentheses != 0 {
        return None;
    }
    if !is_trim || (is_trim && !current_token.is_empty()) {
        tokens.push(current_token.clone());
    }
    Some(tokens)
}

pub fn str_escape(str: &str) -> String {
    let mut result = String::new();
    let mut is_escape = false;
    for c in str.chars() {
        if is_escape {
            result.push(c);
            is_escape = false;
        } else {
            match c {
                '\\' => {
                    is_escape = true;
                }
                _ => result.push(c),
            }
        }
    }
    result
}
