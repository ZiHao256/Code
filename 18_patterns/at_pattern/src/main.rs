struct Point{
    x: i32
}

fn main() {
    let p = Point{x:10};

    match p{
        Point{x: 0..=5} => println!("0..=5"),
        Point{x: z @ 6..=10} => println!("{}",z),
        Point{x} => println!("{}",x)
    }
}
