// 用 ? 运算符简化 match 表达式

use std::io::{self, Read};
use std::fs::File;

fn main() {
    
    let file_name = get_string();

    let res = read_username_from_file(file_name);

    dbg!(res);

}

fn get_string() -> String {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("input error");

    input.trim().to_string()
}

fn read_username_from_file(file_name: String) -> Result<String, io::Error> {
    let mut username = String::new();    
    
    // let mut file = File::open(file_name)?;
    
    // file.read_to_string(&mut username)?;

    File::open(file_name)?.read_to_string(&mut username)?;

    Ok(username)

}