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

    if let Coin::Quarter(state) = a {
        println!("{:?}",state);
    }else{
        println!("not Quarte");
    }


}
