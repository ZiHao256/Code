use std::collections::HashMap;

fn main() {
    let field = String::from("favourite");
    let color = String::from("blue");

    let mut h1 = HashMap::new();

    h1.insert(field, color);

    dbg!(&h1);

    // 访问hash值

    let f1 = String::from("favourite");

    dbg!(h1.get(&f1));

    // 遍历
    for (key, value) in &h1{
        println!("{} {}",key,value);
    }
    dbg!(h1);
}

