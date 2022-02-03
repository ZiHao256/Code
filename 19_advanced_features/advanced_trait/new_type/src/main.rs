
use std::fmt;
use std::fmt::Formatter;

pub struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // write!(f, "({}, {}, {})", self.a, self.b, self.c);
        for item in self.0.iter(){
            write!(f,"{}, ",item);
        }
        Ok(())
    }
}

fn main() {
    let mut w = Wrapper(vec!["1".to_string(),"2".to_string(),"3".to_string()]);

    println!("{}",w);


}
