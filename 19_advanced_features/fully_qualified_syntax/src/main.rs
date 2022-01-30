trait Animal{
    fn baby_name();
}

struct Dog;

impl Dog {
    fn baby_name() {
        println!("Dog");
    }
}

impl Animal for Dog {
    fn baby_name() {
        println!("Animal");
    }
}

fn main() {
    let dog = Dog;

    Dog::baby_name();

    <Dog as Animal>::baby_name();
}
