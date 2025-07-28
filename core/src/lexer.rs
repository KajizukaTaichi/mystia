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
    let mut is_escape = false;
    let mut is_comment = false;

    let chars: Vec<String> = input.chars().map(String::from).collect();
    let mut index = 0;

    fn include_letter(query: &str, chars: &Vec<String>, idx: usize) -> bool {
        chars
            .clone()
            .get(idx..idx + query.chars().count())
            .map(|i| query == i.concat())
            .unwrap_or(false)
    }

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
            current_token.push_str(&c);
            is_escape = false;
            index += 1;
        } else if ["(", "[", "{"].contains(&c.as_str()) && !in_quote {
            if is_split && in_parentheses == 0 {
                tokens.push(current_token.clone());
                current_token.clear();
            }
            current_token.push_str(c.as_str());
            in_parentheses += 1;
            index += 1;
        } else if [")", "]", "}"].contains(&c.as_str()) && !in_quote {
            current_token.push_str(c.as_str());
            in_parentheses = in_parentheses.checked_sub(1)?;
            index += 1;
        } else if c == "\"" {
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

pub fn is_identifier(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    let mut chars = name.chars();
    let first_char = chars.next().unwrap();
    if !UnicodeXID::is_xid_start(first_char) {
        return false;
    }
    if !chars.all(UnicodeXID::is_xid_continue) {
        return false;
    }
    if RESERVED.contains(&name) {
        return false;
    }
    if !name.is_ascii() {
        return false;
    }
    true
}

pub fn str_format(input: &str) -> Option<Vec<String>> {
    let mut tokens: Vec<String> = Vec::new();
    let mut current_token = String::new();
    let mut in_parentheses: usize = 0;
    let mut is_escape = false;

    for c in input.chars() {
        if is_escape {
            current_token.push(c);
            is_escape = false;
        } else {
            match c {
                '{' => {
                    if in_parentheses == 0 {
                        if !current_token.is_empty() {
                            tokens.push(current_token.clone());
                        }
                        current_token = c.to_string();
                    } else {
                        current_token.push(c)
                    }
                    in_parentheses += 1;
                }
                '}' => {
                    current_token.push(c);
                    in_parentheses = in_parentheses.checked_sub(1)?;
                    if in_parentheses == 0 {
                        if !current_token.is_empty() {
                            tokens.push(current_token.clone());
                        }
                        current_token.clear();
                    }
                }
                '\\' => {
                    current_token.push(c);
                    is_escape = true;
                }
                _ => current_token.push(c),
            }
        }
    }

    // Syntax error check
    if is_escape || in_parentheses != 0 {
        return None;
    }
    if !current_token.is_empty() {
        tokens.push(current_token.clone());
        current_token.clear();
    }
    Some(tokens)
}
