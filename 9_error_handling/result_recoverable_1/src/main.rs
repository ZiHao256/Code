// 示例 9-4：使用 match 表达式处理可能会返回的 Result 成员

use std::fs::File;

fn main() {
    
    let f = File::open("hello.txt");

    // let mut s = String::new();

    let f = match f{
        Ok(f) => f,
        Err(e) => panic!("{}",e)
    };

}
