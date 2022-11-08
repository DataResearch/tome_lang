use crate::lexer::lexer::Lexer;
use crate::token::{Token, KEYWORDS};
use crate::iter::BlockingIter;
use crate::predicate;
use crate::lexer::{
    numeric_lexer::NumericLexer,
    string_literal_lexer::StringLiteralLexer,
    bracket_lexer::BracketLexer,
    delimeter_lexer::DelimeterLexer,
    operator_lexer::OperatorLexer
};

use std::iter::{Iterator, Peekable};

pub struct LexicalTokenIterator<T: Iterator<Item=char>> {
    code: Peekable<T>
}

impl<T> LexicalTokenIterator<T> 
    where T: Iterator<Item=char> {

    pub fn new(value: T) -> LexicalTokenIterator<T> {
        LexicalTokenIterator{
            code: value.peekable()
        }
    }

}

impl<T> Iterator for LexicalTokenIterator<T> 
    where T: Iterator<Item=char> {

    type Item = Token;

    // Produces a single new token from the internals of the Tokenzier
    // TODO (@CodingChris): This function can definetly be split up in sub functions / methods.
    // I don't like the return Some(Token::<Type>(<Value>)) stuff. Can be better when split up in smaller units.
    // This might then also cleanup some bugs in parsing currently marked with "TODO".
    fn next(&mut self) -> Option<Self::Item> {

        // trim whitespaces
        self.code.blocking_skip(predicate::is_whitespace);

        // check if there is something to evaluate
        // TODO (@Christian): Code is borrowed mutably here - which requires char to be used
        // as a copy type instead of a reference. The usages of & in the predicates look weired now.
        if let Some(&x) = self.code.peek() {

            // Parse numeric token
            if predicate::is_numeric(&x) {
                if let Ok(token) = NumericLexer::lexing(&mut self.code) {
                    return Some(token);
                }
                else {
                    return Some(Token::Unknown);
                }
            }

            // test for string literals
            if x == '"' {
                if let Ok(token) = StringLiteralLexer::lexing(&mut self.code) {
                    return Some(token);
                }
                else {
                    return Some(Token::Unknown);
                }
            }
            
            // test for dot operation as it is special
            if x == '.' {
                let _ = self.code.next();
                return Some(Token::Dot);
            }

            if predicate::is_bracket(&x) {
                if let Ok(token) = BracketLexer::lexing(&mut self.code) {
                    return Some(token);
                }
                else {
                    return Some(Token::Unknown);
                }
            }

            if predicate::is_delimeter(&x) {
                if let Ok(token) = DelimeterLexer::lexing(&mut self.code) {
                    return Some(token);
                }
                else {
                    return Some(Token::Unknown);
                }
            }

            // operators
            if predicate::is_symbol(&x) {
                if let Ok(token) = OperatorLexer::lexing(&mut self.code) {
                    return Some(token);
                }
                else {
                    return Some(Token::Unknown);
                }
            }

            // test for keywords and literals (literals have their own predicate)
            if predicate::is_alpha(&x) {
            
                let keying_list = self.code.blocking_take(|x| predicate::is_alphanumeric(x) || x == &'_');
                let words: String = keying_list.into_iter().collect();

                if KEYWORDS.contains(&words.as_ref()) {
                    return Some(Token::Keyword(words));
                }
                else {
                    return Some(Token::Identifier(words));
                }

            }
            
            // no parsing case matched - resulting in an unknown token
            self.code.next(); // skip a single token - we might find something useful in the next round over
            return Some(Token::Unknown)

        }
        else {
            return None;
        }

    }

}
