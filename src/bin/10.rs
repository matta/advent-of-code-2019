use ordered_float::OrderedFloat;
use std::{
    collections::BTreeMap,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn point(x: i32, y: i32) -> Point {
    Point { x, y }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, scalar: i32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Div<i32> for Point {
    type Output = Self;

    fn div(self, scalar: i32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

fn distance(p1: Point, p2: Point) -> f32 {
    let dx = (p2.x - p1.x) as f32;
    let dy = (p2.y - p1.y) as f32;

    // Calculate the distance between the two points.
    (dx * dx + dy * dy).sqrt()
}

#[derive(PartialEq, Clone)]
struct BoolGrid {
    grid: Vec<Vec<bool>>,
}

impl BoolGrid {
    fn new(grid: Vec<Vec<bool>>) -> BoolGrid {
        if grid.is_empty() {
            panic!("empty grids are invalid")
        }
        if grid.iter().any(|v| v.is_empty()) {
            panic!("empty grid rows are invalid")
        }
        if grid.iter().any(|v| v.len() != grid[0].len()) {
            panic!("grid rows of unequal length are invalid")
        }
        BoolGrid { grid }
    }

    fn height(&self) -> i32 {
        self.grid.len() as i32
    }

    fn width(&self) -> i32 {
        self.grid[0].len() as i32
    }

    fn get(&self, p: Point) -> bool {
        self.grid[p.y as usize][p.x as usize]
    }

    fn clear(&mut self, p: Point) {
        self.grid[p.y as usize][p.x as usize] = false;
    }
}

impl std::fmt::Debug for BoolGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "BoolGrid {{")?;
        for row in self.grid.iter() {
            for cell in row.iter() {
                write!(
                    f,
                    "{}",
                    match cell {
                        true => '#',
                        false => '.',
                    }
                )?;
            }
            write!(f, " ")?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

fn parse_line(line: &str) -> Vec<bool> {
    line.trim()
        .chars()
        .map(|char| match char {
            '#' => true,
            '.' => false,
            _ => panic!("Invalid character in input string: {}", char),
        })
        .collect()
}

fn parse_grid(string: &str) -> BoolGrid {
    BoolGrid::new(string.trim().split('\n').map(parse_line).collect())
}

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 {
        b.abs()
    } else if b == 0 {
        a.abs()
    } else {
        gcd(b % a.abs(), a.abs())
    }
}

fn erase_line(grid: &mut BoolGrid, start: Point, exemplar: Point) {
    let delta = exemplar - start;
    let gcd = gcd(delta.x, delta.y);
    let step = delta / gcd;
    let mut curr = start;
    while curr.x >= 0 && curr.x < grid.width() && curr.y >= 0 && curr.y < grid.height() {
        grid.clear(curr);
        curr = curr + step
    }
}

fn count_asteroids(center: Point, mut grid: BoolGrid) -> i32 {
    grid.clear(center);
    let mut sum = 0;
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let p = Point { x, y };
            if !grid.get(p) {
                continue;
            }
            sum += 1;
            erase_line(&mut grid, center, p);
        }
    }
    sum
}

fn part_one_counts(grid: &BoolGrid) -> Vec<Vec<i32>> {
    let mut counts: Vec<Vec<i32>> = Vec::new();
    for y in 0..grid.height() {
        counts.push(Vec::new());
        for x in 0..grid.width() {
            let p = Point { x, y };
            let count = if grid.get(p) {
                count_asteroids(p, grid.clone())
            } else {
                0
            };
            counts[y as usize].push(count)
        }
    }
    counts
}

fn part_one_best_count(input: &str) -> i32 {
    let grid = parse_grid(input);
    let counts = part_one_counts(&grid);
    let max = counts.iter().flatten().max().unwrap();
    *max
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(part_one_best_count(input))
}

fn angle(p1: Point, p2: Point) -> f32 {
    let vector = p2 - p1;
    let vector = point(-vector.y, vector.x);

    let radians = match (vector.y as f32).atan2(vector.x as f32) {
        x if x < 0.0 => x + 2.0 * std::f32::consts::PI,
        x => x,
    };

    radians * 180.0 / std::f32::consts::PI
}

fn part_two_compute(input: &str) -> Option<Point> {
    let grid = parse_grid(input);
    let counts = part_one_counts(&grid);
    let mut max_count = 0;
    let mut max_point = Point { x: 0, y: 0 };
    for (y, row) in counts.iter().enumerate() {
        for (x, count) in row.iter().enumerate() {
            if *count > max_count {
                max_count = *count;
                max_point = Point {
                    x: x as i32,
                    y: y as i32,
                };
            }
        }
    }

    type Distances = BTreeMap<OrderedFloat<f32>, Point>;
    type Angles = BTreeMap<OrderedFloat<f32>, Distances>;

    let mut angles: Angles = BTreeMap::new();

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let p = Point { x, y };
            if p == max_point || !grid.get(p) {
                continue;
            }
            let angle = angle(max_point, p);
            let distance = distance(max_point, p);
            angles
                .entry(OrderedFloat(angle))
                .or_default()
                .insert(OrderedFloat(distance), p);
        }
    }

    let mut count = 0;
    while count < 200 {
        for distances in angles.values_mut() {
            if let Some((_, point)) = distances.pop_first() {
                count += 1;
                if count == 200 {
                    return Some(point);
                }
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<i32> {
    part_two_compute(input).map(|point| point.x * 100 + point.y)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use super::*;

    #[test]
    fn test_parse() {
        let input = " .. \n## \n#.\n.#\n  \n";
        assert_eq!(
            parse_grid(input),
            BoolGrid::new(vec![
                vec![false, false],
                vec![true, true],
                vec![true, false],
                vec![false, true]
            ])
        );
    }

    #[test]
    fn test_part_one_counts() {
        assert_eq!(
            part_one_counts(&parse_grid(".#..#\n.....\n#####\n....#\n...##")),
            vec![
                [0, 7, 0, 0, 7],
                [0, 0, 0, 0, 0],
                [6, 7, 7, 7, 5],
                [0, 0, 0, 0, 7],
                [0, 0, 0, 8, 7]
            ]
        );
        assert_eq!(part_one_best_count(".#..#\n.....\n#####\n....#\n...##"), 8);
        assert_eq!(part_one_best_count("......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####"), 33);
        assert_eq!(
            part_one_best_count(
                r#"
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
"#
            ),
            210
        );
    }

    #[test]
    fn test_angle() {
        assert_float_eq!(
            angle(Point { x: 26, y: 28 }, Point { x: 26, y: 9 }),
            0.0,
            abs <= f32::EPSILON
        );
        assert_float_eq!(
            angle(Point { x: 26, y: 28 }, Point { x: 5, y: 28 }),
            270.0,
            abs <= f32::EPSILON
        );
        assert_float_eq!(angle(point(0, 0), point(1, -1)), 45.0, abs <= f32::EPSILON);
        assert_float_eq!(angle(point(1, 2), point(2, 1)), 45.0, abs <= f32::EPSILON);
    }

    #[test]
    fn test_part_two() {
        let input = r#"
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
"#;
        let grid = parse_grid(input);
        assert!(!grid.get(point(12, 0)));
        assert!(grid.get(point(12, 1)));
        assert_eq!(part_two(input), Some(802));
    }
}
