use std::fmt;

mod guess;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BoundError {
    value: u32,
    min: u32,
    max: u32,
}

impl fmt::Display for BoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value out of range: {}. Range min: {}. Range max: {}.", self.value, self.min, self.max)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BoundedU32<const MIN: u32, const MAX: u32>(u32);

impl<const MIN: u32, const MAX: u32> BoundedU32<MIN, MAX> {
    pub const fn new(value: u32) -> Result<Self, BoundError> {
        if value >= MIN && value <= MAX {
            Ok(Self(value))
        } else {
            Err(BoundError{value, min: MIN, max: MAX})
        }
    }
}

// Type alias with defined value constraint
type Element = BoundedU32<1, 100>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounded_u32() {
        let value = Element::new(10);
        let x = value.unwrap().0;
        assert_eq!(x, 10);
    }

    #[test]
    #[should_panic]
    fn test_bound_panic() {
        let value = Element::new(9_910_000);
        // This   should panic panic because value is out of range
        let _x = value.expect("Error creating bounded element");
    }

    #[test]
    fn test_bound_fallback() {
        let value = Element::new(997890000);
        let x = value.unwrap_or( Element::new(1).unwrap());
        assert_eq!(x.0, 1);
    }
}

fn main() {
    println!("Hello, bounded values!");
    let value = Element::new(10);
    let x = value.unwrap().0;
    println!("Bounded x = {}", x);

    // This panics with "Value out of range" because value is way to large to fit range
    // let _out_of_bound_value = Element::new(10000);

    // graceful error handling
    let err_val = Element::new(997890000);
    match err_val {
        Ok(x) => {    println!("x = {}", x.0);  }
        Err(err) => {eprintln!("Error: {}", err)}
    }
}
