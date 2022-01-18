enum SpreadsheetCell{
    Int(i32),
    FLoat(f64),
    Text(String)
}

fn main() {
    let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::FLoat(10.12),
];

    for i in &row{
        match i{
            SpreadsheetCell::FLoat(f) => println!("{}",f),
            SpreadsheetCell::Int(i) => println!("{}",i),
            SpreadsheetCell::Text(s) => println!("{}",s)
        }
    }
}
