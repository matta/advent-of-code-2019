use std::collections::{HashMap, HashSet, VecDeque};

use aoc2019::intcode;

#[derive(Debug)]
struct Mover {
    input: Option<i64>,
    output: Option<i64>,
}

impl intcode::ComputerIO for Mover {
    fn input(&mut self) -> i64 {
        self.input.take().unwrap()
    }

    fn output(&mut self, value: i64) {
        assert!(self.output.is_none());
        self.output = Some(value);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Terrain {
    Wall,
    Open,
    OxygenSystem,
}

impl TryFrom<i64> for Terrain {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Terrain::Wall),
            1 => Ok(Terrain::Open),
            2 => Ok(Terrain::OxygenSystem),
            _ => Err(format!("Invalid terrain value: {}", value)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Command {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Command {
    fn apply_to(&self, p: Point) -> Point {
        match *self {
            Command::North => Point::new(p.x, p.y - 1),
            Command::South => Point::new(p.x, p.y + 1),
            Command::West => Point::new(p.x - 1, p.y),
            Command::East => Point::new(p.x + 1, p.y),
        }
    }
}

impl From<Command> for i64 {
    fn from(movement_command: Command) -> Self {
        movement_command as i64
    }
}

fn move_droid(computer: &mut intcode::Computer, command: Command) -> Terrain {
    let mut input: VecDeque<i64> = [command.into()].iter().copied().collect();
    if let Some(value) = computer.run(&mut input) {
        return value.try_into().unwrap();
    }
    panic!("computer finished without any output")
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn neighbors(&self) -> PointNeighborIterator {
        PointNeighborIterator::new(*self)
    }
}

struct PointNeighborIterator {
    point: Point,
    current_direction: i32,
}

impl PointNeighborIterator {
    fn new(point: Point) -> PointNeighborIterator {
        PointNeighborIterator {
            point,
            current_direction: 0,
        }
    }
}

impl Iterator for PointNeighborIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let neighbor_point = match self.current_direction {
            0 => Point {
                x: self.point.x,
                y: self.point.y + 1,
            },
            1 => Point {
                x: self.point.x,
                y: self.point.y - 1,
            },
            2 => Point {
                x: self.point.x + 1,
                y: self.point.y,
            },
            3 => Point {
                x: self.point.x - 1,
                y: self.point.y,
            },
            _ => return None,
        };

        self.current_direction += 1;

        Some(neighbor_point)
    }
}

#[derive(Debug)]
struct ShipMap {
    terrain: HashMap<Point, Terrain>,
}

impl ShipMap {
    fn get_terrain(&self, point: Point) -> Option<Terrain> {
        self.terrain.get(&point).copied()
    }

    fn set_terrain(&mut self, point: Point, terrain: Terrain) {
        self.terrain.insert(point, terrain);
    }
}

fn pick_command(droid: Point, map: &ShipMap) -> Option<Command> {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();

    let unmapped = |p: &Point| map.get_terrain(*p).is_none();

    // Every point yet to be mapped that is directly reachable from an open
    // square is a goal.
    for (open_point, _) in map.terrain.iter().filter(|(_, &t)| t == Terrain::Open) {
        for unmapped_neighbor in open_point.neighbors().filter(unmapped) {
            if seen.insert(unmapped_neighbor) {
                queue.push_back(unmapped_neighbor);
            }
        }
    }

    while let Some(p) = queue.pop_front() {
        if let Some(command) = compute_movement_command(droid, p) {
            return Some(command);
        }

        for open_neighbor in p
            .neighbors()
            .filter(|n| matches!(map.get_terrain(*n), Some(Terrain::Open)))
        {
            if seen.insert(open_neighbor) {
                queue.push_back(open_neighbor);
            }
        }
    }

    None
}

fn compute_distance(start: Point, finish: Point, map: &ShipMap) -> i32 {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((finish, 0));
    seen.insert(finish);

    while let Some((point, distance)) = queue.pop_front() {
        if point == start {
            return distance;
        }

        for open_neighbor in point
            .neighbors()
            .filter(|n| matches!(map.get_terrain(*n), Some(Terrain::Open)))
        {
            if seen.insert(open_neighbor) {
                queue.push_back((open_neighbor, distance + 1));
            }
        }
    }

    todo!("finish compute_distance()")
}

fn compute_movement_command(from: Point, to: Point) -> Option<Command> {
    if from.x == to.x {
        if from.y + 1 == to.y {
            return Some(Command::South);
        }
        if from.y - 1 == to.y {
            return Some(Command::North);
        }
    }
    if from.y == to.y {
        if from.x + 1 == to.x {
            return Some(Command::East);
        }
        if from.x - 1 == to.x {
            return Some(Command::West);
        }
    }
    None
}

#[allow(dead_code)]
fn print_map(droid: Point, map: &ShipMap) {
    let mut min_x = droid.x;
    let mut max_x = droid.x;
    let mut min_y = droid.y;
    let mut max_y = droid.y;

    for p in map.terrain.keys() {
        min_x = std::cmp::min(min_x, p.x);
        min_y = std::cmp::min(min_y, p.y);
        max_x = std::cmp::max(max_x, p.x);
        max_y = std::cmp::max(max_y, p.y);
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let point = Point::new(x, y);
            if point == droid {
                print!("D");
            } else {
                match map.get_terrain(point) {
                    None => print!(" "),
                    Some(Terrain::Wall) => print!("#"),
                    Some(Terrain::Open) => print!("."),
                    Some(Terrain::OxygenSystem) => print!("O"),
                }
            }
        }
        println!();
    }
}

struct ExploreResult {
    map: ShipMap,
    oxygen_system: Point,
}

fn explore_ship(program_text: &str) -> ExploreResult {
    let mut computer = intcode::Computer::parse(program_text);
    let mut droid = Point::new(0, 0);
    let mut oxygen_system = None;
    let mut map = ShipMap {
        terrain: HashMap::new(),
    };
    map.set_terrain(droid, Terrain::Open);

    #[allow(unused_variables)]
    let mut moves = 0;
    while let Some(command) = pick_command(droid, &map) {
        moves += 1;
        let terrain = move_droid(&mut computer, command);
        let target = command.apply_to(droid);
        map.set_terrain(target, terrain);
        match terrain {
            Terrain::Wall => {}
            Terrain::Open => {
                droid = target;
            }
            Terrain::OxygenSystem => {
                droid = target;
                oxygen_system = Some(target);
            }
        }
        // print!("{esc}c", esc = 27 as char);
        // println!("droid at {:?} -> {:?} -> {:?}", droid, command, terrain);
        // print_map(droid, &map);
        // std::thread::sleep(std::time::Duration::from_millis(20));
    }

    let oxygen_system = oxygen_system.unwrap();
    ExploreResult { map, oxygen_system }
}

fn find_oxygen_system(program_text: &str) -> i32 {
    let explored = explore_ship(program_text);
    compute_distance(Point::new(0, 0), explored.oxygen_system, &explored.map)
}

fn compute_oxygen_fill_time(program_text: &str) -> i32 {
    let explored = explore_ship(program_text);

    let mut max_distance = 0;
    let mut queue = VecDeque::new();
    let mut seen: HashSet<Point> = HashSet::new();
    let maybe_push =
        |point, distance, queue: &mut VecDeque<(Point, i32)>, seen: &mut HashSet<Point>| {
            if seen.insert(point) {
                queue.push_back((point, distance));
            }
        };
    maybe_push(explored.oxygen_system, 0, &mut queue, &mut seen);

    while let Some((point, distance)) = queue.pop_front() {
        max_distance = distance;
        for open_neighbor in point
            .neighbors()
            .filter(|n| matches!(explored.map.get_terrain(*n), Some(Terrain::Open)))
        {
            maybe_push(open_neighbor, distance + 1, &mut queue, &mut seen);
        }
    }
    max_distance
}

fn part_one(input: &str) -> i32 {
    find_oxygen_system(input)
}

fn part_two(input: &str) -> i32 {
    compute_oxygen_fill_time(input)
}

fn main() {
    let input = include_str!("../inputs/15.txt");
    assert_eq!(part_one(input), 240);
    assert_eq!(part_two(input), 322);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
