use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number：{}",secret_number);

    
    loop {

        println!("Please input a number：");

        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess).expect("wrong");
    
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {println!("Please type a number"); continue;},
        };
        
        println!("The number you enter：{}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Equal => {println!("You Win!"); break;},
            Ordering::Less => println!("too less"),
            Ordering::Greater => println!("too greater")
        }

    }
    
}
