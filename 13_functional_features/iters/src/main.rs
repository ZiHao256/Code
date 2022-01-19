fn main() {
    let v1 = vec![1,2,3,4];

    let mut v1_iter = (v1).iter();

    println!("{}", v1_iter.next().unwrap());

    dbg!(&v1_iter);

    let v1_iter: Vec<i32> = v1_iter.map(|x| x+1).collect();

    dbg!(v1_iter);

}
