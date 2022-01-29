fn main() {
    let mut setting_value = Some(String::from("hello"));
    let new_setting_value = Some(String::from("world"));

    match (&setting_value, &new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

}
