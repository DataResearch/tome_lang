
#[derive(Debug, Clone)]

// TODO (@CodingChris): replace bracket and delimeter with type enums
// don't use strings - this is pretty uncool down the road
pub enum Token {
    Operator(String),
    IntegerLiteral(String),
    FloatingPointLiteral(String),
    StringLiteral(String),
    Identifier(String),
    Bracket(String),
    Keyword(String),
    Delimiter(String),
    Dot,
    
    // Control Tokens
    Unknown,
    EOF

}

pub type TokenStream = Vec<Token>;

// TODO (@CodingChris): do we want to recognize some textual
// representations as operators? i.e. and, or, not, eq, ...
pub const OPERATORS: &[&str] = &[
    // logical operators
    "==", ">=", "<=", "=>", "=<", ">", "<", "<=>",

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
    "?", "_", "%%", "°"];

pub const BRACKETS: &[&str] = &["(", ")", "{", "}", "[", "]"];

pub const DELIMETERS: &[&str] = &[",", ":", ";", "\"", "'"];

pub const KEYWORDS: &[&str] = &[
    // user code regions
    "mod",

    // introduce datastorage
    "const", "let",

    // type related keywords
    "fn", "struct", "class", "union", "enum",
    "int", "float", "bool",

    // TODO (@CodingChris): maybe I want to support operator overloading sometimes?
    "operator",

    // values
    "false", "true", "null",

    // conditionals 
    "if", "else", "elif",
    "switch", "match",

    // loops
    "while", "for", "loop",

    // async programming
    "async", "await",
    
    "final", "static", "void", "public", "private", "protected", "interface", "unsafe", "mutable", "mut"];
