fn main() {
    // 1
    let mut s1 = String::new();

    // 2
    let mut s2 = "hello world".to_string();

    // 3
    let mut s3 = String::from("hello world");

    dbg!(&s1);
    dbg!(&s2);
    dbg!(&s3);
}
