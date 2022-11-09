use crate::token::token;

pub fn is_whitespace(x: &char) -> bool {
    match x {
        ' ' => true,
        '\r' => true,
        '\t' => true,
        '\n' => true,
        _ => false
    }
}

pub fn is_numeric(x: &char) -> bool {
    match x {
        '0'..='9' => true,
        _ => false
    }
}

pub fn is_alpha(x: &char) -> bool {
    match x {
        'a'..='z' => true,
        'A'..='Z' => true,
        _ => false
    }
}

pub fn is_alphanumeric(x: &char) -> bool {
    is_alpha(x) || is_numeric(x)
}

pub fn is_symbol(x: &char) -> bool {
    let symbol_list = &['+', '-', '*', '/', '|', '&', '^',
        '~', '!', '#', '?', '=', '%', '.', '°', '@'];
    symbol_list.contains(x) || is_bracket(x) || is_delimeter(x)
}

pub fn is_bracket(x: &char) -> bool {
    let mut tmp = [0; 4];
    token::Bracket::is_bracket(x.encode_utf8(&mut tmp))
}

pub fn is_delimeter(x: &char) -> bool {
    let mut tmp = [0; 4];
    token::Delimeter::is_delimeter(x.encode_utf8(&mut tmp))
}