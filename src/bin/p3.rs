extern crate primes;

struct Eratosthenes(Vec<u64>);

impl Eratosthenes {
    pub fn empty() -> Self {
        Eratosthenes(vec![])
    }
}

impl Iterator for Eratosthenes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.len() {
            0 => { self.0.push(2); }
            1 => { self.0.push(3); }
            _ => {
                let mut candidate = (*self.0.last().unwrap()) + 2;
                while self.0.iter().any(|prime| candidate % prime == 0) {
                    candidate += 2;
                }
                self.0.push(candidate);
            }
        }

        Some(*self.0.last().unwrap())
    }
}

fn factors(n: u64) -> Vec<u64> {
    let mut decomp = n;
    let mut factors = vec![];

    for prime in Eratosthenes::empty() {
        if decomp % prime == 0 {
            factors.push(prime);

            while decomp % prime == 0 {
                decomp /= prime;
            }
        }
        if prime >= decomp {
            break;
        }
    }

    factors
}

fn largest_prime(n: u64) -> u64 {
    *factors(n).last().unwrap()
}

fn main() {
    let subject = 600851475143;
    let largest = largest_prime(subject);
    println!("{}", largest);
    assert_eq!(largest, 6857);
}

#[cfg(test)]
mod tests {
    use super::*;
    use primes::is_prime;

    #[test]
    fn test_eratosthenes() {
        let sieve = Eratosthenes::empty();

        for i in sieve.take(100) {
            assert!(is_prime(i));
        }
    }
}
