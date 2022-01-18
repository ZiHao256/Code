use std::io;
#[derive(Debug)]
struct Guess {
    value: i32,
}

impl Guess {
    fn new(num: i32) -> Guess {
        if num > 100 || num < 0 {
            panic!("wrong guess");
        } else {
            Guess { value: num }
        }
    }

    fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    dbg!(get_guess().value());
}

fn get_guess() -> Guess {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("wrong input");

    let num: i32 = input.trim().parse().expect("not a number");

    let guess = Guess::new(num);

    guess
}
