#![allow(dead_code)]

use std::iter::Iterator;

struct Fib(Vec<u64>);

impl Fib {
    pub fn new() -> Fib {
        Fib(vec![])
    }
}

impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let n = match self.0.len() {
            0 => { 1 }
            1 => { 2 }
            _ => { self.0[self.0.len() - 1] + self.0[self.0.len() - 2] }
        };

        self.0.push(n);
        Some(n)
    }
}

fn main() {
    let seq = Fib::new();
    let summed = seq
        .take_while(|n| n <= &4_000_000)
        .filter(|n| n % 2 == 0)
        .fold(0, |acc, n| acc + n);

    println!("{}", summed);
    assert_eq!(summed, 4613732);
}
