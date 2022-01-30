trait Pilot{
    fn fly(&self);
}
trait Wizard{
    fn fly(&self);
}

struct Human(String);

impl Human {
    fn fly(&self){
        println!("{} fly as Human", self.0);
    }
}

impl Pilot for Human {
    fn fly(&self){
        println!("{} fly as Pilot", self.0);
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("{} fly as Wizard",self.0);
    }
}
fn main() {
    let h = Human(String::from("马子豪"));
    h.fly();
    Wizard::fly(&h);
    Pilot::fly(&h);
}
