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

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}
fn internal_adder(a: i32, b: i32) -> i32  {
    a + b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[should_panic(expected = "I expected error")]    
    #[ignore]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 10,
        };

        assert!(larger.can_hold(&smaller), "Cant hold us");
    }
}
