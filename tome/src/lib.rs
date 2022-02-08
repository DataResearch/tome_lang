#![allow(dead_code)]
mod iter;
pub mod file;
pub mod token;
pub mod tokenizer;
pub mod predicate;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
