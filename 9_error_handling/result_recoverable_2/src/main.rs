use std::io::ErrorKind;
use std::fs::File;

fn main() {
    let mut f = File::open("hello.txt");

    let mut f = match f{
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("{}",e)
            },
            other => panic!("open file warning {:?}",other)
        }
    };

    
}
