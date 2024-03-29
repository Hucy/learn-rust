#[derive (Debug)]
struct Rectangle {
    width:u32,
    height:u32
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn square(size:u32) -> Rectangle {
        Rectangle {width:size,height:size}
    }
}


impl Rectangle{
    fn can_hold(&self,other:&Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let width = 30;
    let height = 50;
    println!("The area of the rectangle is {} square pixels.", area(width,height));

    let rect = (30,50);
    println!("The area of the rectangle is {} square pixels.", tuple_area(rect));
    
    let rect = Rectangle {
        width:30,
        height:50
    };
    println!("rect is {:#?}", rect);
    println!("The area of the rectangle is {} square pixels.", struct_area(&rect));

    println!("The area of the rectangle is {} square pixels.", rect.area());
    let rect1 = Rectangle{
        width:20,
        height:40
    };
    
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect.can_hold(&rect1));
    println!("Can rect1 hold rect3? {}", rect.can_hold(&rect3));

    let square = Rectangle::square(40);
    println!("square is {:#?}",square);
}

fn area(width:u32,height:u32) -> u32 {
    width * height
}

fn tuple_area(dimensions:(u32,u32)) -> u32{
    dimensions.0 * dimensions.1
}

fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

