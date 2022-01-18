enum Message{
    Quit,
    Move{
        x:u32,
        y:u32
    },
    Write(String),
    ChangeColor(i32,i32,i32)
}

impl Message{
    fn call(&self){

    }
}

fn main() {
    println!("Hello, world!");
}
