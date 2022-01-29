fn return_closure() -> Box<dyn Fn(i32)->i32>{
    Box::new(|x|{x+1})
}

fn main() {
    let x = return_closure();
    let x = x(1);
    println!("{}", x);
}
