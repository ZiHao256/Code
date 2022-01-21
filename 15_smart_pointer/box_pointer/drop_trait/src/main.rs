struct CustomSmartPointer{
    data: String
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("Dropping data: {}",self.data);
    }
}

fn main() {
    
    let a = CustomSmartPointer{data: String::from("hello world")};
    let b = CustomSmartPointer{data: String::from("another data")};

    drop(a);

    println!("create data");
}
