fn rev1(n: u64, acc: u64) -> u64 {
    match n {
        0 => acc,
        _ => rev1(n / 10, acc * 10 + (n % 10))
    }
}

fn is_palindrome(n: u64) -> bool {
    n == rev1(n, 0)
}

fn main() {
    let largest = (100..1000).flat_map(|a| (100..1000).map(move |b| (a, b)))
        .map(|(a, b)| a * b)
        .filter(|n| is_palindrome(*n))
        .max().unwrap();

    assert_eq!(906609, largest);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome1() {
        assert!(is_palindrome(10301));
    }

    #[test]
    fn test_is_palindrome2() {
        assert!(is_palindrome(1));
    }

    #[test]
    fn test_is_palindrome3() {
        assert!(is_palindrome(11));
    }

    #[test]
    fn test_is_palindrome4() {
        assert!(is_palindrome(4224));
    }
}
