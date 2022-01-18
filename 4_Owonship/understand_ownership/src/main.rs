fn main() {
    let a = String::from("hello world");
    let b = &a;

    greet(&a);
    greet(&b);

    let c = b;

    greet(&a);
    greet(&b);
    greet(&c);
}
fn greet(s: &String){
    println!("{}", s);
}