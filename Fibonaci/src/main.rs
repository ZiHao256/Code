// 生成 n 阶斐波那契数列。
fn main() {
    let mut input = String::new();

    println!("Please input n");

    std::io::stdin().read_line(&mut input).expect("wrong");

    let number:u32 = match input.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("wrong number");
            1
        }
    };

    for i in 1..number+1{
        println!("{}",fib(i));
    }

}

fn fib(n:u32) -> u32{
    if n==1 || n==2{
        return 1
    }
    return fib(n-2) + fib(n-1)
}