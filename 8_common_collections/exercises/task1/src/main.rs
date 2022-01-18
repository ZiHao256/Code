// 给定一系列数字，使用 vector 并返回这个列表的平均数（mean, average）、
// 中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；这里哈希 map 会很有帮助）。

use std::collections::HashMap;

fn main() {
    let mut v: Vec<f32> = Vec::new();

    read_nums(&mut v);

    for i in &v {
        print!(" {}", i);
    }

    let average = get_average(&v);

    let medium = get_medium(&v);

    let mode = get_mode(v);

    println!("\n{} {} {}", average, medium, mode);
}
fn read_nums(v: &mut Vec<f32>) {
    for i in 0..10 {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("wrng input");

        let x: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => 0.0,
        };

        v.push(x);
    }
}

fn get_average(v: &Vec<f32>) -> f32 {
    let mut sum: f32 = 0.0;
    for i in v {
        sum += i;
    }
    sum / (v.len() as f32)
}

fn get_medium(v: &Vec<f32>) -> f32 {
    let mut v2 = v.clone();
    v2.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v2[v2.len() / 2]
}

fn get_mode(v: Vec<f32>) -> f32 {
    let mut hash = HashMap::new();

    let mut v2 = v.clone();

    for f in v2{
      let mut count = hash.entry(f).or_insert(0);
      *count += 1;
    }

    let mut ans:(f32,u32) = (0.0, 0);

    for (k,v) in hash{
        if(v >= ans.1){
            ans.0 = k;
            ans.1 = v;
        }
    }
    ans.0

}
