fn main() {
    let mut s1 = String::from("hello");

    dbg!(&s1);

    // 1 
    s1.push_str(" world");

    dbg!(&s1);

    // 2
    let s2 = " world";

    s1.push_str(s2);

    dbg!(&s1);

    dbg!(s2);

    // push
    s1.push('🌏');

    dbg!(&s1);
}
