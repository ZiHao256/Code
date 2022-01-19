fn main() {
    let v1 = vec![1,2,3,4];

    let mut v1_iter = v1.into_iter();

    println!("{}", v1_iter.next().unwrap());

    dbg!(v1_iter.next().unwrap());

}
