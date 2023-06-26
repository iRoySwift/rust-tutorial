#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U,
}

impl<T, U> Rectangle<T, U>
where
    T: std::ops::Mul<U, Output = V>,
    U: Copy,
    V: Copy,
{
    fn area<V>(&self) -> V
    where
        T: Into<V>,
        U: Into<V>,
    {
        let width: V = self.width.into();
        let height: V = self.height.into();
        width * height
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
