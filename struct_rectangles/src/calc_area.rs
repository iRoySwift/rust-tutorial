#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U,
}

impl<T: std::ops::Mul<U, Output = ()>, U> Rectangle<T, U> {
    fn area(&self) {
        self.width * self.height
    }
}

pub fn run() {
    let rectangle = Rectangle {
        width: 5,
        height: 10.5,
    };

    let area = rectangle.area();
    println!("矩形的面积为: {:?}", area);
}
