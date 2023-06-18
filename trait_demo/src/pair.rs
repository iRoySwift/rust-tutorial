struct Pair<T> {
    y: T,
    x: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x == self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The smallest member is y = {}", self.y);
        }
    }
}
