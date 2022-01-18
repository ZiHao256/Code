fn main() {

    eat_at_restaurant();

}

// 声明 front_of_house 模块，就像模块定义在该作用域一样使用该模块
mod front_of_house;
mod back_of_house; 

pub fn eat_at_restaurant() {
    use crate::{front_of_house::hosting, back_of_house::Appetizer};


    // Absolute path
    hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut break_fast = back_of_house::Breakfast::summer("Rye");

    break_fast.toast = String::from("change");

    println!("break_fast {}",break_fast.toast);

    let order1 = Appetizer::Salad;
    let order2 = Appetizer::Soup;

}
