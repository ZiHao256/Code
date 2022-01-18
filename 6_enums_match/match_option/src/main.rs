fn main() {
    
    let a = Some(1);
    let b:Option<i32> = None;

    println!("{:?} {:?}", a, b);

    let a = plus_one(a);
    let b = plus_one(b);

    println!("{:?} {:?}", a, b);



}

fn plus_one(a: Option<i32>) -> Option<i32>{
    match a{
        Some(num) => Some(num+1),
        None => None
    }
}