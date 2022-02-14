//! Project Euler 5
//!
//! 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//!
//! What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

extern crate num;

use num::integer::gcd;

fn main() {

    // lcm(a, b, c) = lcm(a, lcm(b, c))

    let least: u64 = (1..=20).fold(1u64, |acc, n|
        (n * acc) / gcd(n, acc)
    );
    println!("{}", least);
}
