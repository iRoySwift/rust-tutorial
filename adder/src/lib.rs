pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Rectangle {
    length: usize,
    width: usize,
}

impl Rectangle {
    pub fn can_hold(&self, other: Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_hold() {
        let larger = Rectangle {
            width: 8,
            length: 7,
        };
        let smaller = Rectangle {
            width: 5,
            length: 1,
        };
        assert!(larger.can_hold(smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            length: 7,
        };
        let smaller = Rectangle {
            width: 5,
            length: 1,
        };
        assert!(smaller.can_hold(larger));
    }
}
