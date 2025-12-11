use std::collections::{HashMap, HashSet, VecDeque};

use aoc2019::intcode::{self, RunState};
use aoc2019::point::Point2D;

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

const COMMANDS: [Command; 4] = [Command::North, Command::South, Command::East, Command::West];

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
    computer.append_input(&[command.into()]);
    if let RunState::BlockedOnOutput = computer.run() {
        return computer.take_output().unwrap().try_into().unwrap();
    }
    panic!("computer finished without any output")
}

type Point = Point2D<i32>;

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

struct ExploreResult {
    map: ShipMap,
    oxygen_system: Point,
    oxygen_system_distance: u32,
}

fn explore_ship(program_text: &str) -> ExploreResult {
    let mut oxygen_system = None;
    let mut oxygen_system_distance = u32::MAX;
    let mut map = ShipMap {
        terrain: HashMap::new(),
    };

    struct FrontierState {
        pos: Point,
        distance: u32,
        computer: intcode::Computer,
    }

    let start = Point::default();
    map.set_terrain(start, Terrain::Open);

    let mut queue = VecDeque::new();
    queue.push_back(FrontierState {
        pos: start,
        distance: 0,
        computer: intcode::Computer::parse(program_text),
    });

    while let Some(state) = queue.pop_front() {
        for command in COMMANDS.iter() {
            let dest = command.apply_to(state.pos);
            if map.get_terrain(dest).is_some() {
                continue;
            }
            let mut computer = state.computer.clone();
            let terrain = move_droid(&mut computer, *command);
            map.set_terrain(dest, terrain);
            if terrain == Terrain::Wall {
                continue;
            }
            let frontier = FrontierState {
                pos: dest,
                distance: state.distance + 1,
                computer,
            };
            if terrain == Terrain::OxygenSystem {
                oxygen_system = Some(dest);
                oxygen_system_distance = frontier.distance;
            }
            queue.push_back(frontier);
        }
    }

    let oxygen_system = oxygen_system.expect("Search should find the oxygen system");
    ExploreResult {
        map,
        oxygen_system,
        oxygen_system_distance,
    }
}

fn find_oxygen_system(program_text: &str) -> u32 {
    explore_ship(program_text).oxygen_system_distance
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
            .cardinal_neighbors()
            .filter(|n| matches!(explored.map.get_terrain(*n), Some(Terrain::Open)))
        {
            maybe_push(open_neighbor, distance + 1, &mut queue, &mut seen);
        }
    }
    max_distance
}

fn part_one(input: &str) -> u32 {
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
