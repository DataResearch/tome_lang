
#[derive(Debug, Clone)]
pub enum Token {
    Operator(String),
    IntegerLiteral(String),
    FloatingPointLiteral(String),
    StringLiteral(String),
    Identifier(String),
    Bracket(char),
    Keywork(String),
    Delimeter(char),
    
    // Control Tokens
    Unknown,
    EOF

}

pub type TokenStream = Vec<Token>;

pub const OPERATORS: &[&str] = &[
    // logical operators
    "==", ">=", "<=", ">", "<",

    // boolean logic & bitwise manipulation
    "~", "|", "&", "!", "||", "&&", "^",

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
    "?", "_", "%%", "°"];

pub const BRACKETS: &[&str] = &["(", ")", "{", "}", "[", "]"];

pub const DELIMETERS: &[&str] = &[",", ":", ";", "\"", "'"];

pub const KEYWORDS: &[&str] = &[
    // user code regions
    "mod",

    // introduce datastorage
    "const", "let",

    // type related keywords
    "fn", "struct", 
    "int", "float", "bool",

    // TODO (@CodingChris): maybe I want to support operator overloading sometimes?
    "operator",

    // truth values
    "false", "true",

    // conditionals 
    "if", "else", "elif",

    // loops
    "while", "for", "loop",

    // async programming
    "async", "await"];

pub const WHITESPACES: &[&str] = &[" ", "\t", "\r", "\n"];
