#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub(crate) struct Point<T, U: PartialOrd + Copy> {
    x: T,
    y: U,
}

impl<T, U: PartialOrd + Copy> Point<T, U> {
    pub(crate) fn new(x: T, y: U) -> Self {
        Point { x, y }
    }
}

impl<T, U: PartialOrd + Copy> Point<T, U> {
    pub(crate) fn x(&self) -> &T {
        return &self.x;
    }

    pub(crate) fn y(&self) -> &U {
        return &self.y;
    }

    pub(crate) fn mixup<V, W: PartialOrd + Copy>(self, other: Point<V, W>) -> Point<T, W> {
        return Point {
            x: self.x,
            y: other.y,
        };
    }
}