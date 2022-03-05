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
    match x {
        '+' | '-' | '*' | '/' => true,
        '|' | '&' | '^' => true,
        ',' | ':' | ';' => true,
        '\'' | '"' => true, 
        '<' | '>' => true,
        '(' | ')' => true, 
        '{' | '}' => true,
        '[' | ']' => true, 
        '~' | '!' => true,
        '#' | '?' => true, // TODO (@CodingChris): leverave # annotated code to be used at compiletime?
        '=' => true,
        '%' => true,
        '.' => true,
        '°' => true,
        _ => false
    }
}

pub fn is_bracket(x: &char) -> bool { 
    match x {
        '<' | '>' => true, // meta programming and generic brackets
        '(' | ')' => true, // function call and precendence
        '{' | '}' => true, // code blocks
        '[' | ']' => true, // lists, arrays, subscripts
        _ => false
    }
}

pub fn is_delimeter(x: &char) -> bool {
    match x {
        ',' | ':' | ';' => true,
        _ => false
    }
}