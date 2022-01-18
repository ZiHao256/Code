fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");

    dbg!(&s1, &s2);

    // 1
    let s3 = s1 +" "+ &s2;

    dbg!(&s3);

    // 2
    let s4 = format!("{} {}","hello",s2);

    dbg!(s2,s4);
}
