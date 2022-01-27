use trait_object::*;

struct Button();

impl Draw for Button{
    fn draw(&self){
        println!("Button");
    }
}

struct SelectBox();

impl Draw for SelectBox{
    fn draw(&self){
        println!("SelectBox");
    }
}

fn main() {
    let screen = Screen{
        components: vec![
            Box::new(Button()) ,
            Box::new(SelectBox()) 
        ]
    };

    screen.run();
}
