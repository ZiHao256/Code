fn main() {
    let favourite_color:Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();


    if let Some(color) = favourite_color {
        
    }else if is_tuesday{

    }else if let Ok(age) = age {
        if age>30{
            println!("ok");
        }else{
            println!("no");
        }
    }
}
