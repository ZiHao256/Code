fn main() {
    let x = Some(10);
    let y = 10;
    match x{
        Some(5) => println!("5"),
        Some(n) if n==y => println!("x==y"),
        _ => println!("ohter")
    }
}
