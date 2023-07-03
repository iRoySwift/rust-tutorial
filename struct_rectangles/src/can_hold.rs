pub fn run() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let sq = Rectangle::square(3);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("rect {:#?}", sq)
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // 关联函数
    // 不以 self 为第一参数的关联函数（因此不是方法）
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn larger_can_hold_small() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller))
    }

    #[test]
    #[should_panic]
    fn small_can_hold_large() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(smaller.can_hold(&larger))
    }
}
