// 面向对象特性：命名的对象、封装、继承
// trait 组合了数据与行为
// 无法为trait对象添加数据

pub trait Draw {
    fn draw(&self);
}

// 使用dyn 把trait转成对象
// Trait对象必须保证对象安全
// 判断是否对象安全
// 方法的返回类型不是Self
// 方法中不包含任何泛型参数
// Clone不属于对象安全
pub trait Clone {
    fn clone(&self) -> Self;
}

pub struct Screen {
    // 动态trait trait转成对象
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// ------------使用泛型------------------
// T Button
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 绘制一个按钮
    }
}
