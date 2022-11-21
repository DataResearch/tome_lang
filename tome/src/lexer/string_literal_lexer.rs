use std::collections::HashMap;
use std::io::Take;
use std::iter::{Peekable, Iterator};
use crate::extensions::iter::{BlockingIter, IteratorStateInfo};
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

enum TakeUntilType {
    IteratorExhaused,
    LastMarkerCompletesLiteral,
    LastMarkerEscaped
}

struct TakeUntilState {
    content: Vec<char>,
    contentType: TakeUntilType
}

pub struct StringLiteralLexer{}

impl StringLiteralLexer {

    fn take_until_literalmarker<I> (iterator: &mut Peekable<I>) -> Option<TakeUntilState>
        where I: Iterator<Item=char>
    {
        let (elements, state) = iterator.blocking_take_until(|x| *x == '"');

        let is_escaped =
            |elements: &Vec<char>| if elements.len() > 1 { elements[elements.len() - 2] == '\\' } else { false };
        let state =
            if state == IteratorStateInfo::OperationExhausted { TakeUntilType::IteratorExhaused }
            else if is_escaped(&elements) { TakeUntilType::LastMarkerEscaped }
            else { TakeUntilType::LastMarkerCompletesLiteral };

        if elements.len() > 0 { Some(TakeUntilState{ content: elements, contentType: state }) }
        else { None }
    }
}

impl Lexer for StringLiteralLexer {

    fn lexing<I> (iterator: &mut Peekable<I>) -> Result<Token, LexicalError> 
        where I: Iterator<Item=char> 
    {
        let replacementMap: HashMap<_, _> =
            collection!{r#"\""# => r#"""#, r#"\\"# => r#"\"#};

        let mut buffer = Vec::new();
        let concat = |content| [buffer, content].concat();

        // literal start marker
        let target = StringLiteralLexer::take_until_literalmarker(iterator);

        match target {
            Some(state) => {

            },
            _ => {
                return Err(LexicalError::UnexpectedEndOfStream);
            }
        };

    }
}
