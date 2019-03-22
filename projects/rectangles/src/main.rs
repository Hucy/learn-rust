#[derive (Debug)]
struct Rectangle {
    width:u32,
    height:u32
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

