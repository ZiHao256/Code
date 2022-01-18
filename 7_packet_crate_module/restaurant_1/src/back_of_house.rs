// 模块back_of_house定义文件

pub struct Breakfast {
    pub toast: String,
    season_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            season_fruit: String::from("peaches"),
        }
    }
}

pub enum Appetizer {
    Soup,
    Salad,
}

fn fix_incorrect_order() {
    cook_order();
    super::front_of_house::serving::take_order();
}

fn cook_order() {}
