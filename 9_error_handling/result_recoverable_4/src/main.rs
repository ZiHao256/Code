// 一个从文件中读取用户名的函数。
// 如果文件不存在或不能读取，这个函数会将这些错误返回给调用它的代码：
use std::fs::File;
use std::io::{self, Read};

fn main() {

    let filename = get_string();

    let res = read_username_from_file(filename);

    dbg!(res);


}

fn get_string() -> String {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("input error");

    input.trim().to_string()
}

fn read_username_from_file(file_name: String) -> Result<String, io::Error> {
    let mut file = File::open(file_name);

    let mut file = match file {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
