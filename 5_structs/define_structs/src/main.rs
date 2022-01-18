struct User {
    email: String,
    name: String,
    phone_number: String,
}
fn main() {
    let user1 = User {
        email: String::from("zihao@rust.com"),
        name: String::from("zihao"),
        phone_number: String::from("151"),
    };
    println!("user1 {} {} {}", user1.email, user1.name, user1.phone_number);

    let mut user2 = build_user("zihao".to_string(), "zihao".to_string());

    user2.email = "hello".to_string();

    println!("usre2 {} {} {}", user2.email, user2.name, user2.phone_number);

    let user3 = User{
        ..user1
    };

    println!("user3 {} {} {}",user3.email, user3.name, user3.phone_number);


}

fn build_user(email: String, name: String) -> User {
    User {
        email,
        name,
        phone_number:  String::from("111")
    }
}
