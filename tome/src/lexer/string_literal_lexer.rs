use std::collections::HashMap;
use std::iter::{Peekable, Iterator};
use crate::extensions::iter::BlockingIter;
use crate::token::token::Token;
use super::lexer::{Lexer, LexicalError};

// TODO (@CodingChris): this should be moved to its own module
macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        core::convert::From::from([$(($k, $v),)*])
    }};
    // set-like
    ($($v:expr),* $(,)?) => {{
        core::convert::From::from([$($v,)*])
    }};
}

pub struct StringLiteralLexer{}

impl Lexer for StringLiteralLexer {
    
    fn lexing<I> (iterator: &mut Peekable<I>) -> Result<Token, LexicalError> 
        where I: Iterator<Item=char> 
    {
        let replacementMap: HashMap<_, _> =
            collection!{r#"\""# => r#"""#, r#"\\"# => r#"\"#};

    }
}
