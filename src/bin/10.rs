use std::ops::{Add, Div, Mul, Sub};

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
        return self.grid.len() as i32;
    }

    fn width(&self) -> i32 {
        return self.grid[0].len() as i32;
    }

    fn get(&self, p: Point) -> bool {
        return self.grid[p.y as usize][p.x as usize];
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
    BoolGrid::new(
        string
            .trim()
            .split("\n")
            .map(|line| parse_line(line))
            .collect(),
    )
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

fn part_one_counts(input: &str) -> Vec<Vec<i32>> {
    let mut counts: Vec<Vec<i32>> = Vec::new();
    let grid = parse_grid(input);
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
    let counts = part_one_counts(input);
    let max = counts.iter().flatten().max().unwrap();
    *max
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(part_one_best_count(input))
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
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
            part_one_counts(".#..#\n.....\n#####\n....#\n...##"),
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
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
