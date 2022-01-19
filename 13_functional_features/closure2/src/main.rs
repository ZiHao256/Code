fn main() {
    let mut x = vec![1,2];

    let equal = |i| i==x;

    let mut y = vec![1,2];

    assert_eq!(equal(y),true);

    dbg!(x);
}
