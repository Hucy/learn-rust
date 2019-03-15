const MAX_NUM:u32 = 1000;

fn main() {
    let mut x = 1;
    println!("The value of x is {}", x);
    x = 2;
    println!("The value of x is {}", x);
    println!("The value of MAX_NUM is {}", MAX_NUM);
    shadowing();
}

fn shadowing() {
    let x = "shadowing";
    println!("shadowing:The value of x is {}", x);
    let x = x.len();
    println!("shadowing:The value of x is {}", x);
}