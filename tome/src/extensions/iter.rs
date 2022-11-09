use std::iter::{Peekable, Iterator};
use std::ops::Fn;

pub trait BlockingIter<I, T>: where I: Iterator<Item=T>{
    fn blocking_skip(&mut self, predicate: impl Fn(&I::Item)->bool);
    fn blocking_take(&mut self, predicate: impl Fn(&I::Item)->bool) -> Vec<I::Item>;
}

impl<I, T> BlockingIter<I, T> for Peekable<I> where I: Iterator<Item=T> {

    fn blocking_skip(&mut self, predicate: impl Fn(&I::Item)->bool) {
        self.blocking_take(predicate);
    }

    fn blocking_take(&mut self, predicate: impl Fn(&I::Item)->bool) -> Vec<I::Item> {
        let mut elements = Vec::new();
        while if let Some(x) = self.peek() { predicate(x) } else { false } {
            // Note: The unwrap here is safe, since it is checked by an if let beforehand
            elements.push(self.next().unwrap());
        }
        elements
    }

}
