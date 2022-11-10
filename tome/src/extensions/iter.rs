use std::iter::{Peekable, Iterator};
use std::ops::Fn;

pub trait BlockingIter<I, T>: where I: Iterator<Item=T>{
    fn blocking_skip(&mut self, predicate: impl Fn(&I::Item)->bool);
    fn blocking_take(&mut self, predicate: impl Fn(&I::Item)->bool) -> Vec<I::Item>;
    /// Takes elements out of the iterator and stores them in a vector until the predicate returns true
    fn blocking_take_until(&mut self, predicate: impl Fn(&I::Item)->bool) -> (Vec<I::Item>, IteratorStateInfo);
}

pub enum IteratorStateInfo {
    OperationCompleted,
    OperationExhausted
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

    fn blocking_take_until(&mut self, predicate: impl Fn(&I::Item)->bool) -> (Vec<I::Item>, IteratorStateInfo) {
        let mut buffer = Vec::new();

        while {
            let entry = self.next();
            let hit;

            match entry {
                None => {return (buffer, IteratorStateInfo::OperationExhausted);},
                Some(x) => {buffer.push(x); hit = buffer.last().unwrap();}
            }

            !predicate(hit)
        } {}

        (buffer, IteratorStateInfo::OperationCompleted)
    }
}
