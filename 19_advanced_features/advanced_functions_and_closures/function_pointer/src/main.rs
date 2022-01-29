fn add_one(value: i32) ->i32{
    value+1
}

fn do_twice(f:fn(i32)->i32, arg:i32) -> i32{
    f(arg) + f(arg)
}

fn main() {
    println!("the ans is {}", do_twice(add_one, 10));
}
