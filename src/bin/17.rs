use core::fmt;
use std::{collections::VecDeque, ops::Add, ops::Sub};

use advent_of_code::intcode::{Computer, ComputerIO};

struct CaptureOutput {
    output: Option<i64>,
}

impl ComputerIO for CaptureOutput {
    fn input(&mut self) -> i64 {
        panic!("CaptureOutput does not support input!")
    }

    fn output(&mut self, value: i64) {
        assert!(self.output.is_none());
        self.output = Some(value);
    }
}

fn is_scaffold(scaffold: &[Vec<bool>], pos: Point) -> bool {
    if pos.x < 0 || pos.y < 0 {
        return false;
    }
    if let Some(row) = scaffold.get(pos.y as usize) {
        if let Some(bit) = row.get(pos.x as usize) {
            return *bit;
        }
    }
    false
}

fn is_alignment_parameter(scaffold: &[Vec<bool>], pos: Point) -> bool {
    is_scaffold(scaffold, pos)
        && is_scaffold(scaffold, pos + Direction::North.delta())
        && is_scaffold(scaffold, pos + Direction::South.delta())
        && is_scaffold(scaffold, pos + Direction::East.delta())
        && is_scaffold(scaffold, pos + Direction::West.delta())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn as_char(&self) -> char {
        match *self {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::East => '>',
            Direction::West => '<',
        }
    }

    fn delta(&self) -> Point {
        match *self {
            Direction::North => Point::new(0, -1),
            Direction::South => Point::new(0, 1),
            Direction::East => Point::new(1, 0),
            Direction::West => Point::new(-1, 0),
        }
    }

    fn turn(&self, turn: Turn) -> Direction {
        match turn {
            Turn::Left => self.left(),
            Turn::Right => self.right(),
        }
    }

    fn left(&self) -> Direction {
        match *self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    fn right(&self) -> Direction {
        match *self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

fn get_map(input: &str) -> (Vec<Vec<bool>>, Point, Direction) {
    let mut c = Computer::parse(input);
    let mut scaffold = Vec::new();
    let mut robot_pos = None;
    let mut robot_dir = None;

    let mut row = Vec::new();
    while let Some(value) = c.run(&mut VecDeque::new()) {
        let value: u32 = value.try_into().expect("bad output number");
        let value = value.try_into().expect("bad output number");
        match value {
            '#' => {
                // scaffold
                row.push(true);
            }
            '.' => {
                // space
                row.push(false);
            }
            '\n' => {
                // new line
                if row.is_empty() {
                    break; // end of output
                }
                scaffold.push(row);
                row = Vec::new();
            }
            '^' => {
                // robot facing up
                robot_dir = Some(Direction::North);
                robot_pos = Some(Point::new(row.len() as i32, scaffold.len() as i32));
                row.push(true);
            }
            _ => panic!("unexpected output from computer: {}", value),
        }
    }

    (scaffold, robot_pos.unwrap(), robot_dir.unwrap())
}

pub fn part_one(input: &str) -> Option<i32> {
    let (scaffold, _pos, _dir) = get_map(input);

    let mut sum: i32 = 0;
    for (y, row) in scaffold.iter().enumerate() {
        let y: i32 = y.try_into().unwrap();
        for (x, bit) in row.iter().enumerate() {
            if *bit {
                let x: i32 = x.try_into().unwrap();
                if is_alignment_parameter(&scaffold, Point::new(x, y)) {
                    sum += x * y;
                }
            }
        }
    }
    Some(sum)
}

fn walk_forward(scaffold: &[Vec<bool>], pos: &Point, dir: &Direction) -> Point {
    let mut pos = *pos;
    loop {
        let next = pos + dir.delta();
        if !is_scaffold(scaffold, next) {
            return pos;
        }
        pos = next;
    }
}

fn manhattan_distance(a: Point, b: Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn pick_turn(scaffold: &[Vec<bool>], pos: Point, dir: &Direction) -> Option<Turn> {
    if is_scaffold(scaffold, pos + dir.left().delta()) {
        return Some(Turn::Left);
    }
    if is_scaffold(scaffold, pos + dir.right().delta()) {
        return Some(Turn::Right);
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Turn {
    Left,
    Right,
}

impl fmt::Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Turn::Left => write!(f, "L"),
            Turn::Right => write!(f, "R"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Movement {
    Distance(u8),
    Turn(Turn),
    Routine(u8),
}

impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Movement::Distance(distance) => write!(f, "{}", distance),
            Movement::Turn(turn) => write!(f, "{}", turn),
            Movement::Routine(num) => {
                let ch = match num {
                    0 => 'A',
                    1 => 'B',
                    2 => 'C',
                    _ => panic!("Invalid routine number: {}", num),
                };
                write!(f, "{}", ch)
            }
        }
    }
}

#[allow(dead_code)]
fn draw(scaffold: &[Vec<bool>], pos: Point, dir: Direction) {
    for (y, row) in scaffold.iter().enumerate() {
        let y: i32 = y.try_into().unwrap();
        for (x, bit) in row.iter().enumerate() {
            let x: i32 = x.try_into().unwrap();
            let point = Point::new(x, y);
            let ch = if point == pos {
                dir.as_char()
            } else if *bit {
                '#'
            } else {
                '.'
            };
            print!("{}", ch);
        }
        println!();
    }
}

fn stumble_around(scaffold: &[Vec<bool>], pos: Point, dir: Direction) -> Vec<Movement> {
    // Walk the map using a simple heuristic:
    //
    // a) walk forward until we can't
    //
    // b) figure out which way to turn from there.
    //
    // c) If there are multiple choices then panic (doesn't seem to happen
    //    in practice). If there are no choices (dead end) then we're done.
    //    otherwise, turn, then go to step a

    let mut route = Vec::new();

    let mut pos = pos;
    let mut dir = dir;
    loop {
        // println!("loop");
        // println!("  pos {:?} dir {:?}", pos, dir);
        let next = walk_forward(scaffold, &pos, &dir);
        let distance = manhattan_distance(pos, next);
        // println!("  walked {} squares to {:?}", distance, next);
        if distance > 0 {
            let d: u8 = distance.try_into().unwrap();
            route.push(Movement::Distance(d));
        }

        pos = next;
        if let Some(turn) = pick_turn(scaffold, next, &dir) {
            route.push(Movement::Turn(turn));
            dir = dir.turn(turn);
        } else {
            break;
        }

        // println!("route = {:?}", route);
        assert!(route.len() < 100);
    }

    route
}

fn encode_route(route: &[Movement]) -> String {
    use std::io::Write;

    let mut v: Vec<u8> = Vec::new();
    let mut first = true;
    for m in route {
        if !first {
            write!(&mut v, ",").unwrap();
        } else {
            first = false;
        }
        write!(&mut v, "{}", m).unwrap();
    }
    String::from_utf8(v).unwrap()
}

fn position_of(needle: &[Movement], haystack: &[Movement]) -> Option<usize> {
    for (i, w) in haystack.windows(needle.len()).enumerate() {
        if w.iter().zip(needle).all(|(a, b)| a == b) {
            return Some(i);
        }
    }
    None
}

fn replace_all(
    route: &[Movement],
    movements: &[Movement],
    routine: Movement,
) -> Option<Vec<Movement>> {
    // println!("\nXXX replace_all");
    // println!("    route =     {}", format_route(route));
    // println!("    movements = {}", format_route(movements));
    // println!("    routine =   {}", format_route(&[routine; 1]));

    if let Some(i) = position_of(movements, route) {
        let mut replaced_route = route.to_vec();
        let mut i = i;
        loop {
            replaced_route.splice(i..i + movements.len(), [routine; 1]);
            if let Some(j) = position_of(movements, &replaced_route) {
                i = j;
            } else {
                break;
            }
        }
        return Some(replaced_route);
    }
    None
}

fn compress2(route: &[Movement], routines: &mut Vec<Vec<Movement>>) -> Option<Vec<Movement>> {
    // println!("compress2 depth {}", routines.len());

    if routines.len() == 3 {
        if route.iter().all(|m| matches!(*m, Movement::Routine(_))) {
            let encoded_route = encode_route(route);
            if encoded_route.len() <= 20 {
                return Some(route.to_vec());
            } else {
                println!("too long for main movement routine: {}", encoded_route);
            }
        } else {
            return None;
        }
    }

    let is_routine = |m: &Movement| matches!(m, Movement::Routine(_));

    for (compressible_start, movement) in route.iter().enumerate() {
        if is_routine(movement) {
            continue;
        }

        let max_compressible_end = route
            .iter()
            .skip(compressible_start)
            .position(is_routine)
            .unwrap_or(route.len());

        for compressible_end in compressible_start + 1..=max_compressible_end {
            let movements = route[compressible_start..compressible_end].to_vec();
            assert!(!movements.iter().any(is_routine));
            if encode_route(&movements).len() > 20 {
                break;
            }
            let routine = Movement::Routine(routines.len().try_into().unwrap());
            if let Some(route) = replace_all(route, &movements, routine) {
                routines.push(movements);
                if let Some(compressed) = compress2(&route, routines) {
                    return Some(compressed);
                } else {
                    routines.pop();
                }
            }
        }
    }

    None
}

fn compress(route: &[Movement]) -> (Vec<Movement>, Vec<Vec<Movement>>) {
    let mut routines = Vec::new();
    let compressed = compress2(route, &mut routines).expect("Failed to compress route.");
    (compressed, routines)
}

pub fn part_two(intcode: &str) -> Option<i64> {
    let (scaffold, pos, dir) = get_map(intcode);

    // draw(&scaffold, pos, dir);

    let route = stumble_around(&scaffold, pos, dir);
    println!("route {}", encode_route(&route));
    println!("route len {}", encode_route(&route).len());

    let (movement_routine, movement_functions) = compress(&route);

    let mut input = encode_route(&movement_routine);
    input.push('\n');
    for x in movement_functions {
        input.push_str(&encode_route(&x));
        input.push('\n');
    }
    let continuous_feed = false;
    input.push_str(&format!("{}\n", if continuous_feed { 'y' } else { 'n' }));

    let mut input: VecDeque<i64> = input
        .as_bytes()
        .iter()
        .map(|&e| -> i64 { e.into() })
        .collect();

    let mut c = Computer::parse(intcode);
    c.poke(0, 2);
    while let Some(out) = c.run(&mut input) {
        if (0..128).contains(&out) {
            let ch = char::from_u32(out as u32).unwrap();
            print!("{}", ch);
        } else {
            return Some(out);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("inputs", 17);
        assert_eq!(part_one(&input), Some(7328));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("inputs", 17);
        assert_eq!(part_two(&input), Some(1289413));
    }
}
