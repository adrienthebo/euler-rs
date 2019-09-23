fn main() {
    let set: Vec<u64> = (1..=20).rev().scan(vec![], |acc, n| {
        if acc.iter().any(|x| x % n == 0) {
            None
        } else {
            acc.push(n);
            Some(n)
        }
    }).collect();

    println!("{:?}", set);

    let smallest = (1 .. ::std::u64::MAX)
        .step_by(20 * 19)
        .find(|n| {
            set
                .iter()
                .rev()
                .all(|d| n % d == 0)
        }).unwrap();

    assert_eq!(smallest, 232_792_560);
}
