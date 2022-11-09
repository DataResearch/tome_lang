#![allow(dead_code)]
pub mod extensions;
pub mod lexer;
pub mod parser;
pub mod token;

pub mod file;
pub mod tokenizer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
