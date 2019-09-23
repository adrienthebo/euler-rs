fn main() {
    let range = 1..1000;

    let matching = range.filter(|x| x % 5 == 0 || x % 3 == 0);
    let reduced  = matching.fold(0, |acc, x| acc + x);

    println!("{:?}", reduced);
    assert_eq!(reduced, 233168);
}
