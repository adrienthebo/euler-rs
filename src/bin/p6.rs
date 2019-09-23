extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigInt;
use num_bigint::ToBigInt;
use num_traits::Zero;

fn sq_sum(start: BigInt, stop: BigInt) -> BigInt {
    let n = (start..=stop).fold(Zero::zero(), |acc, n| acc + n);
    n * n
}

fn sum_sq(start: BigInt, stop: BigInt) -> BigInt {
    (start..=stop).fold(Zero::zero(), |acc, n| acc + n * n)
}

fn delta() -> BigInt {
    let start = 1_u32.to_bigint().unwrap();
    let stop = 100_u32.to_bigint().unwrap();
    sum_sq(start, stop) - sq_sum(start, stop)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta() {
        assert_eq!(delta().into(), 25164150.to_bigint().unwrap());
    }
}
