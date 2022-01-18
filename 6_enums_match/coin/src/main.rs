#[derive(Debug)]
enum State{
    A,
    B,
    C
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(State)
}

fn main() {
    let a = Coin::Quarter(State::A);

    println!("{}",value_in_cents(&a));


}

fn value_in_cents(coin: &Coin) -> u32{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        }
    }
}