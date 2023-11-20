use std::cmp::Ord;
use std::fmt;
use std::ops::Add;
use std::ops::Sub;

use num::One;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardinalDirection {
    North,
    South,
    East,
    West,
}

impl CardinalDirection {
    pub fn negate(&self) -> Self {
        match *self {
            CardinalDirection::North => CardinalDirection::South,
            CardinalDirection::South => CardinalDirection::North,
            CardinalDirection::East => CardinalDirection::West,
            CardinalDirection::West => CardinalDirection::East,
        }
    }
}

pub const CARDINAL_DIRECTIONS: [CardinalDirection; 4] = [
    CardinalDirection::North,
    CardinalDirection::South,
    CardinalDirection::East,
    CardinalDirection::West,
];

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

impl<T: Add<Output = T> + Sub<Output = T> + Ord + Copy + TryFrom<usize> + TryInto<usize> + One>
    Point2D<T>
{
    /// Creates a new [`Point2D<T>`].
    pub fn new(x: T, y: T) -> Point2D<T> {
        Point2D { x, y }
    }

    /// Creates a new [`Point2D<T>`] from `usize` arguments.  Returns None
    /// if the integral conversion fails.
    pub fn usize_new(x: usize, y: usize) -> Option<Point2D<T>> {
        Some(Self::new(x.try_into().ok()?, y.try_into().ok()?))
    }

    /// Returns this point as a pair (x, y) of usize.
    pub fn as_usize_pair(&self) -> Option<(usize, usize)> {
        Some((self.x.try_into().ok()?, self.y.try_into().ok()?))
    }

    pub fn manhattan_distance(&self, other: Self) -> T {
        abs_difference(self.x, other.x) + abs_difference(self.y, other.y)
    }

    pub fn neighbors(&self) -> NeighborsIterator<T> {
        NeighborsIterator::new(*self)
    }

    pub fn cardinal_neighbors(&self) -> CardinalNeighborsIterator<T> {
        CardinalNeighborsIterator::new(*self)
    }

    pub fn cardinal_neighbor(&self, dir: CardinalDirection) -> Self {
        let one = T::one();
        match dir {
            CardinalDirection::North => Self::new(self.x, self.y - one),
            CardinalDirection::South => Self::new(self.x, self.y + one),
            CardinalDirection::East => Self::new(self.x + one, self.y),
            CardinalDirection::West => Self::new(self.x - one, self.y),
        }
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

impl<T: fmt::Display> fmt::Display for Point2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: fmt::Display> fmt::Debug for Point2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct NeighborsIterator<T> {
    point: Point2D<T>,
    current_direction: i32,
}

impl<T> NeighborsIterator<T> {
    fn new(point: Point2D<T>) -> NeighborsIterator<T> {
        NeighborsIterator {
            point,
            current_direction: 0,
        }
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + One + Copy> Iterator for NeighborsIterator<T> {
    type Item = Point2D<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let neighbor_point = {
            let one = T::one();
            match self.current_direction {
                0 => Self::Item {
                    x: self.point.x,
                    y: self.point.y + one,
                },
                1 => Self::Item {
                    x: self.point.x,
                    y: self.point.y - one,
                },
                2 => Self::Item {
                    x: self.point.x + one,
                    y: self.point.y,
                },
                3 => Self::Item {
                    x: self.point.x - one,
                    y: self.point.y,
                },
                4 => Self::Item {
                    x: self.point.x + one,
                    y: self.point.y + one,
                },
                5 => Self::Item {
                    x: self.point.x - one,
                    y: self.point.y - one,
                },
                6 => Self::Item {
                    x: self.point.x + one,
                    y: self.point.y - one,
                },
                7 => Self::Item {
                    x: self.point.x - one,
                    y: self.point.y + one,
                },
                _ => return None,
            }
        };

        self.current_direction += 1;

        Some(neighbor_point)
    }
}

pub struct CardinalNeighborsIterator<T> {
    point: Point2D<T>,
    current_direction: i32,
}

impl<T> CardinalNeighborsIterator<T> {
    fn new(point: Point2D<T>) -> CardinalNeighborsIterator<T> {
        CardinalNeighborsIterator {
            point,
            current_direction: 0,
        }
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + From<i32> + Copy> Iterator
    for CardinalNeighborsIterator<T>
{
    type Item = Point2D<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let neighbor_point = {
            match self.current_direction {
                0 => Self::Item {
                    x: self.point.x,
                    y: self.point.y + T::from(1),
                },
                1 => Self::Item {
                    x: self.point.x,
                    y: self.point.y - T::from(1),
                },
                2 => Self::Item {
                    x: self.point.x + T::from(1),
                    y: self.point.y,
                },
                3 => Self::Item {
                    x: self.point.x - T::from(1),
                    y: self.point.y,
                },
                _ => return None,
            }
        };

        self.current_direction += 1;

        Some(neighbor_point)
    }
}
