fn main() {
    let mut stack: Vec<i32> = Vec::new();

    stack.push(1);
    stack.push(2);

    while let Some(value) = stack.pop(){
        println!("{}",value);
    }
}
