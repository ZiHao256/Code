// 换算关系为F=9/5c+32, 或C=5/9(F-32)。
fn main() {
    let mut input = String::new();

    println!("please input F or C");

    std::io::stdin().read_line(&mut input).expect("wrong");

    let ans:i32 = 

    if(input.trim() == "C"){
        let mut number = String::new();

        println!("please input a number");
    
        std::io::stdin().read_line(&mut number).expect("wrong");

        let number: i32 = match number.trim().parse(){
            Ok(num) => num,
            Err(_) => 0
        };
        C_2_F(number)

    }else if(input.trim() == "F"){
        let mut number = String::new();

        println!("please input a number");
    
        std::io::stdin().read_line(&mut number).expect("wrong");

        let number: i32 = match number.trim().parse(){
            Ok(num) => num,
            Err(_) => 0
        };
        F_2_C(number)
    }else{
        println!("try again");
        0
    };

    println!("the res is {}", ans);
}

fn F_2_C(F:i32) -> i32{
    5/9*(F-32)
}

fn C_2_F(C:i32) -> i32{
    9/5*C+32
}