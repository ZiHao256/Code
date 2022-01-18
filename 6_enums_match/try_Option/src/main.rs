fn main() {
    let a = Some(9);
    let b:Option<u32> = None;

    match b{
        Some(num) => println!("{}", num),
        None => println!("None")
    }

    println!("{:?}",a);
}
