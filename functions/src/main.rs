fn main() {
    println!("Hello, world!");

    println!("{}",another_function(5,'😀'));
}

fn another_function(value:i32, unit_label:char) -> i32 {
    println!("Another function:{} {}",value,unit_label);
    5
}
