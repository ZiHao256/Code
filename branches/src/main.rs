fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let ans = if number != 0{
        "not zero"
    }else{
        "zero"
    };
    println!("ans is : {}",ans);

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("wrong");
    
    let res = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    println!("res {}",res);


}
