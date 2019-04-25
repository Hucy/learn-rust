enum List {
    Cons(i32, Box<List>),
    Nil,
}

use std::rc::Rc;
enum ListRc{
    Cons(i32, Rc<ListRc>),
    Nil,
}


use crate::List::{Cons, Nil};
use crate::ListRc::{Cons as RcCons, Nil as RcNil};

fn main() {
    // Box
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // 自定义智能指针
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // 函数和方法解引用强制多态
    let name = MyBox::new(String::from("Rust"));
    hello(&name);

    // 引用计数
    let a = Rc::new(RcCons(5,Rc::new(RcCons(10,Rc::new(RcNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = RcCons(3,Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        
        let _c = RcCons(4,Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello {}", name)
}
