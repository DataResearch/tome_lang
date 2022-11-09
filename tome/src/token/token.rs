#[derive(Debug, Clone)]
pub enum Token {
    Operator(String),
    // Intermediate element can be combined by a <NumericCombination><Dot><NumericCombination> into a floating point literal
    // is an integer literal otherwise
    Numeric(String),
    StringLiteral(String),
    Identifier(String),
    Bracket(Bracket),
    Keyword(String),
    Delimiter(Delimeter),
    Dot,

    // Control Tokens
    EOF,
}

pub enum TokenError {
    TokenNotInClass,
    UnknownToken,
}

pub const OPERATORS: &[&str] = &[
    // logical operators
    "==", "!=", ">=", "<=", "=>", "=<", ">", "<", "<=>",

    // boolean logic & bitwise manipulation
    "~", "|", "&", "^",
    "!", "||", "&&",

    // math operators where // is integer devision and % is remainder / modulo
    "+", "-", "*", "/", "//", "%",
    "+=", "-=", "*=", "/=", "//=", "%=",

    // shift operators
    // shift and fill with 0
    ">>", "<<",
    // shift and fill with 1
    ">>/", "<</",
    // shift but roll over values shifted out
    ">>>", "<<<",

    // assignment operator
    "=",

    // range operators
    "..", "..=",

    // compiletime instruction
    "#",

    // reserved operators with no clear use
    "?", "_", "%%", "°", "@", "->"];

#[derive(Debug, Clone)]
pub enum Bracket {
    LPARENTHESES,
    RPARENTHESES,
    LCURLY,
    RCURLY,
    LSQUARE,
    RSQUARE,
    LANGLE,
    RANGLE
}

impl Bracket {
    pub fn is_bracket(value: &str) -> bool {
        matches!(value, "(" | ")" | "{" | "}" | "[" | "]" | "<" | ">")
    }
}

impl TryFrom<&char> for Bracket {
    type Error = TokenError;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Bracket::LPARENTHESES),
            ')' => Ok(Bracket::RPARENTHESES),
            '{' => Ok(Bracket::LCURLY),
            '}' => Ok(Bracket::RCURLY),
            '[' => Ok(Bracket::LSQUARE),
            ']' => Ok(Bracket::RSQUARE),
            '<' => Ok(Bracket::LANGLE),
            '>' => Ok(Bracket::RANGLE),
            _ => Err(TokenError::TokenNotInClass)
        }
    }
}

impl TryFrom<&str> for Bracket {
    type Error = TokenError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "(" => Ok(Bracket::LPARENTHESES),
            ")" => Ok(Bracket::RPARENTHESES),
            "{" => Ok(Bracket::LCURLY),
            "}" => Ok(Bracket::RCURLY),
            "[" => Ok(Bracket::LSQUARE),
            "]" => Ok(Bracket::RSQUARE),
            "<" => Ok(Bracket::LANGLE),
            ">" => Ok(Bracket::RANGLE),
            _ => Err(TokenError::TokenNotInClass)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Delimeter {
    COMMA,
    COLON,
    SEMICOLON,
    QUOTE,
    SINGLEQUOTE
}

impl TryFrom<&char> for Delimeter {
    type Error = TokenError;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            ',' => Ok(Delimeter::COMMA),
            ':' => Ok(Delimeter::COLON),
            ';' => Ok(Delimeter::SEMICOLON),
            '\'' => Ok(Delimeter::QUOTE),
            '\"' => Ok(Delimeter::SINGLEQUOTE),
            _ => Err(TokenError::TokenNotInClass)
        }
    }
}

impl TryFrom<&str> for Delimeter {
    type Error = TokenError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "," => Ok(Delimeter::COMMA),
            ":" => Ok(Delimeter::COLON),
            ";" => Ok(Delimeter::SEMICOLON),
            "\'" => Ok(Delimeter::QUOTE),
            "\"" => Ok(Delimeter::SINGLEQUOTE),
            _ => Err(TokenError::TokenNotInClass)
        }
    }
}

impl Delimeter {
    pub fn is_delimeter(value: &str) -> bool {
        matches!(value, "," | ":" | ";" | "\"" | "'")
    }
}

pub const KEYWORDS: &[&str] = &[
    // user code regions
    "mod",

    // introduce datastorage
    "const", "let",

    // type related keywords
    "fn", "func", "struct", "class", "union", "enum",
    "int", "float", "bool",

    // specify ineger and float sizing
    "uint8", "int8", "uint16", "int16",
    "uint32", "int32", "uint64", "int64",
    "f32", "f64",

    // TODO (@CodingChris): maybe I want to support operator overloading sometimes?
    "operator", "ctor", "dtor", "move", "copy",

    // values
    "false", "true", "null",

    // conditionals
    "if", "else", "elif",
    "switch", "match",

    // loops
    "while", "for", "loop",

    // async programming
    "async", "await",
    "final", "static", "void", "public", "private", "protected", "interface",
    "unsafe", "mutable", "mut", "ref", "virtual", "override"];
