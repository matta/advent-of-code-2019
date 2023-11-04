use std::cmp::Ord;
use std::ops::Add;
use std::ops::Sub;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

fn abs_difference<T: Sub<Output = T> + Ord>(x: T, y: T) -> T {
    if x < y {
        y - x
    } else {
        x - y
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Ord + Copy> Point2D<T> {
    pub fn new(x: T, y: T) -> Point2D<T> {
        Point2D { x, y }
    }

    pub fn manhattan_distance(&self, other: Self) -> T {
        abs_difference(self.x, other.x) + abs_difference(self.y, other.y)
    }
}

// Notice that the implementation uses the associated type `Output`.
impl<T: Add<Output = T>> Add for Point2D<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point2D<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
