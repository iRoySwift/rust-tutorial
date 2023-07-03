#[derive(Debug)]
struct Rectangle<T> {
    width: T,
    height: T,
}

impl<T> Rectangle<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    fn area(&self) -> T {
        self.width * self.height
    }
}

pub fn run() {
    let rectangle = Rectangle {
        width: 5,
        height: 10,
    };

    let area = rectangle.area();
    println!("矩形的面积为: {:?}", area);
}
