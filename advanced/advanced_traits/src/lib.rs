use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters; // overwrite associated (return) type

    // overwrite rhs argument
    fn add(self, other: Meters) -> Self::Output {
        // convert meter to millimeters during addition
        Millimeters(self.0 + (other.0 * 1000))
    }
}
