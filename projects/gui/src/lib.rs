pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// 使用泛型和 trait bound
// 如果只需要同质（相同类型）集合，则倾向于使用泛型和 trait bound，因为其定义会在编译时采用具体类型进行单态化。
// 另一方面，通过使用 trait 对象的方法，一个 Screen 实例可以存放一个既能包含 Box<Button>，也能包含 Box<TextField> 的 Vec
pub struct ScreenT<T: Draw> {
    pub components: Vec<T>,
}

impl<T> ScreenT<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}


pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 实际绘制按钮的代码
    }
}
