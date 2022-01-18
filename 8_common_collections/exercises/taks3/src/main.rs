// 使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。
// 例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。
// 接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工按照字典序排列的列表。

use std::collections::HashMap;
use std::io;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    let mut counter = 3;

    while counter != 0 {
        let input = get_string();

        let mut temp: Vec<String> = Vec::new();

        for i in input.split_whitespace() {
            temp.push(i.to_string());
        }

        let names = company.entry(temp[3].clone()).or_insert(vec![]);

        names.push(temp[1].clone());

        counter -= 1;
    }

    dbg!(&company);
}

fn get_string() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("wrong input");

    input.trim().to_string()
}
