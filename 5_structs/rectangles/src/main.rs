#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle{
    // 方法
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool{
        self.width>rect2.width && self.height>rect2.height
    }

    // 非方法
    fn square(l: u32) -> Rectangle{
        Rectangle{
            width:l,
            height: l
        }
    }
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // 第一种打印Debug的方式
    println!("第一种：{:#?}",rect1);


    // 第二种打印 Debug 的方式
    
    dbg!(&rect1);
    
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));   


    let square1 = Rectangle::square(30);
    println!("正方形： {:?}", square1);
    


}


