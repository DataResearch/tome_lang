use std::iter::{Peekable, Iterator};
use crate::token::Token;
use super::lexer::{Lexer, LexicalError};

struct LiteralWalkerState {
    dataset: Vec<char>,
    found_escape_sequence: bool,
    continue_parsing: bool,
    first_round: bool
}

impl LiteralWalkerState {

    fn new() -> LiteralWalkerState {
        LiteralWalkerState {
            dataset: vec![],
            found_escape_sequence: false,
            continue_parsing: true,
            first_round: true
        }
    }

}

pub struct StringLiteralLexer{}

impl Lexer for StringLiteralLexer {
    
    fn lexing<I> (iterator: &mut Peekable<I>) -> Result<Token, LexicalError> 
        where I: Iterator<Item=char> 
    {
        let mut processing_state = LiteralWalkerState::new();
        
        while processing_state.continue_parsing {

            let value = iterator.next();

            if let Some(character) = value {
                
                processing_state.dataset.push(character);

                // In the first round
                if processing_state.first_round {

                    // check that it really is the start of a string.
                    if character != '"' {
                        return Err(LexicalError::UnexpectedSymbol(character));
                    }

                    // don't test for the next rounds over as a new "-character
                    // is the end of the string
                    processing_state.first_round = false;
                    continue;
                }

                // triggers when we start an escape sequence
                // when we are already in an escape sequence
                // this function doesn't trigger
                if character == '\\' && !processing_state.found_escape_sequence {
                    processing_state.found_escape_sequence = true;
                    continue;
                }

                // when we are inside an escape sequence we take a char
                // and can continue to parse the string as required
                if processing_state.found_escape_sequence {
                    processing_state.found_escape_sequence = false;
                    continue;
                }

                // when we find an "-character we stop the parsing of
                // a string literal - for a \" sequence we never enter this
                // processing function
                if character == '"' {
                    // TODO (@CodingChris): I think we can employ the usage of a break keyword.
                    processing_state.continue_parsing = false;
                    continue;
                }

            }
            else {
                return Err(LexicalError::UnexpectedEndOfStream())
            }

        }

        Ok(Token::StringLiteral(processing_state.dataset.into_iter().collect()))
    }
}
