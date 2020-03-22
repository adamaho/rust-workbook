#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(n: u32) -> u32 {
    n + 2
}

#[cfg(test)]
mod tests {
    use super::Rectangle;
    use super::add_two;

    #[test]
    fn larger_can_hold_smaller() {
        let large_rec = Rectangle {
            width: 10,
            height: 10,
        };
        let small_rec = Rectangle {
            width: 5,
            height: 5,
        };

        assert!(large_rec.can_hold(&small_rec));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let large_rec = Rectangle {
            width: 10,
            height: 10,
        };
        let small_rec = Rectangle {
            width: 5,
            height: 5,
        };

        assert!(!small_rec.can_hold(&large_rec));
    }

    #[test]
    fn it_adds_two() {
        let n = 5;

        assert_eq!(add_two(n), 7);
    }
}
