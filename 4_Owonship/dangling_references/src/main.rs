fn main() {
    println!("hello");

    let s1 = dangle();
    println!("{}",s1);

}

fn dangle() -> String{
    let s = String::new();
    s
}
