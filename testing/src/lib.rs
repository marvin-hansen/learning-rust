#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    return a + 2;
}

pub fn greetings(name: &str) -> String {
    return format!("Hello: {}", name);
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater or equal to 1")
        } else if value > 100 {
            panic!("Guess value must less than or equal to 100")
        }

        return Guess { value };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // import everything from upper scope
    use super::test_utils::*;

    #[test]
    fn test_it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_larger_can_hold_smaller() {
        let (larger, smaller) = test_utils::get_small_large_rectangle();
        // larger can hold smaller
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn test_smaller_cannot_hold_larger() {
        let (larger, smaller) = test_utils::get_small_large_rectangle();
        // smaller cannot hold larger
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn test_add_two() {
        // Test equality
        assert_eq!(add_two(2), 4);

        // nest not equal
        assert_ne!(add_two(2), 3);
    }

    #[test]
    fn test_greeting_contains_name() {
        let name = "Carol";
        let result = greetings(name);
        assert!(result.contains(name))
    }

    #[test]
    #[should_panic] // only passes when it panics
    fn test_greeting_does_not_contains_name() {
        let name = "Carol";
        let result = greetings("");
        assert!(
            result.contains(name),
            "Greeting did not contains, name, value was {} ",
            result,
        )
    }

    #[test]
    fn test_new_valid_guess() {
        let g = Guess::new(3);
        assert_eq!(g.value, 3);

        let g = Guess::new(5);
        assert_eq!(g.value, 5);

        let g = Guess::new(7);
        assert_eq!(g.value, 7);
    }

    #[test]
    #[should_panic(expect = "Guess value must be greater or equal to 1")] // only passes when it panics
    fn test_new_invalid_guess_below_1() {
        let g = Guess::new(-1);
    }

    #[test]
    #[should_panic(expect = "Guess value must less than or equal to 100")] // only passes when it panics
    fn test_new_invalid_guess_above_100() {
        let g = Guess::new(200);
    }
}


pub mod test_utils {
    // import everything from upper scope
    use super::*;

    pub fn get_small_large_rectangle() -> (Rectangle, Rectangle) {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        return (larger, smaller);
    }
}