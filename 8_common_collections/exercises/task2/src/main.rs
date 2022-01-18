// 将字符串转换为 Pig Latin，
// 也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，所以 “first” 会变成 “irst-fay”。
// 元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。牢记 UTF-8 编码！

fn main() {
    let s1 = get_string();

    println!("{}", get_pig_latin(&s1));
}

fn get_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("wrong input");
    input.trim().to_string()
}

fn get_pig_latin(s: &String) -> String {
    let yuanyin = vec!["A", "a"];

    let first = &s[0..1];
    let rest = &s[1..];

    if (!yuanyin.contains(&first)) {
        format!("{}-{}{}", rest, first, "ay")
    } else {
        format!("{}-hay", s)
    }
}
