#![allow(dead_code, unused_variables)]

use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::hash::Hash;

use aoc2019::point::Point2D;
use pathfinding::prelude::dijkstra;

const INPUT: &str = include_str!("../inputs/18.txt");

type Point = Point2D<i32>;
type Grid = Vec<Vec<Cell>>;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
struct KeySet {
    mask: u32,
}

impl fmt::Debug for KeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "{")?;
        for ch in 'A'..='Z' {
            let key: KeySet = ch.try_into().unwrap();
            if self.contains(key) {
                write!(f, "{}", ch)?
            }
        }
        write!(f, "{}", '}')
    }
}

impl KeySet {
    fn len(&self) -> u32 {
        self.mask.count_ones()
    }

    fn is_empty(&self) -> bool {
        self.mask == 0
    }

    fn union(&self, other: KeySet) -> KeySet {
        KeySet {
            mask: self.mask | other.mask,
        }
    }

    fn contains(&self, other: KeySet) -> bool {
        (self.mask & other.mask) == other.mask
    }
}

impl TryFrom<u32> for KeySet {
    type Error = &'static str;

    fn try_from(mask: u32) -> Result<Self, Self::Error> {
        let max_mask = (1 as u32) << (26 as u32);
        if mask.count_ones() == 1 && mask <= max_mask {
            Ok(KeySet { mask })
        } else {
            Err("invalid key mask value")
        }
    }
}

impl TryFrom<char> for KeySet {
    type Error = &'static str;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        let offset = match ch {
            'A'..='Z' => (ch as u32) - ('A' as u32),
            'a'..='z' => (ch as u32) - ('a' as u32),
            _ => return Err("invalid key character"),
        };
        assert!(offset < 26);
        let mask = (1 as u32) << (offset as u32);
        let key: KeySet = mask.try_into()?;
        Ok(key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Key(KeySet),
    Door(KeySet),
    Entrance,
    Open,
    Wall,
}

impl Cell {
    fn floor_key(&self) -> KeySet {
        match self {
            Cell::Key(key) => *key,
            _ => KeySet::default(),
        }
    }

    fn door_key(&self) -> KeySet {
        match self {
            Cell::Door(key) => *key,
            _ => KeySet::default(),
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = &'static str;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '@' => Ok(Cell::Entrance),
            '#' => Ok(Cell::Wall),
            '.' => Ok(Cell::Open),
            'A'..='Z' => {
                let mask: KeySet = (1_u32 << ((ch as u32) - ('A' as u32))).try_into()?;
                Ok(Cell::Door(mask))
            }
            'a'..='z' => {
                let mask: KeySet = (1_u32 << ((ch as u32) - ('a' as u32))).try_into()?;
                Ok(Cell::Key(mask))
            }
            _ => Err("invalid character for cell"),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Agent {
    points: Vec<Point>,
    keys: KeySet,
}

impl Agent {
    fn new() -> Agent {
        Agent::default()
    }
}

impl fmt::Display for Agent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "<{:?} ", self.points)?;
        for ch in 'A'..='Z' {
            let key: KeySet = ch.try_into().unwrap();
            let c = if self.keys.contains(key) { ch } else { '.' };
            write!(f, "{}", c)?;
        }
        write!(f, ">")
    }
}

fn parse_grid(input: &str) -> Grid {
    let grid: Grid = input
        .trim()
        .split_ascii_whitespace()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| {
                    let cell: Cell = ch.try_into().unwrap();
                    cell
                })
                .collect::<Vec<Cell>>()
        })
        .collect();

    grid
}

fn grid_get(grid: &Grid, pos: Point) -> Cell {
    if pos.y < 0 && pos.x < 0 {
        return Cell::Wall;
    }
    grid.get(pos.y as usize)
        .and_then(|row| row.get(pos.x as usize).copied())
        .unwrap_or(Cell::Wall)
}

fn grid_is_wall(grid: &Grid, pos: Point) -> bool {
    match grid_get(grid, pos) {
        Cell::Wall => true,
        cell @ (Cell::Open | Cell::Entrance | Cell::Key(_) | Cell::Door(_)) => false,
    }
}

fn grid_get_non_wall_neighbors(grid: &Grid, pos: Point) -> Vec<(Cell, Point)> {
    let mut neighbors = Vec::new();

    if !grid_is_wall(grid, pos) {
        let directions: [Point; 4] = [
            Point::new(0, -1),
            Point::new(0, 1),
            Point::new(-1, 0),
            Point::new(1, 0),
        ];

        for neighbor_dir in directions {
            let neighbor_pos = pos + neighbor_dir;
            if !grid_is_wall(grid, neighbor_pos) {
                neighbors.push((grid_get(grid, neighbor_pos), neighbor_pos));
            }
        }
    }

    neighbors
}

#[derive(Debug, Clone, Copy)]
struct ReachableKey {
    pos: Point,
    floor_key: KeySet,
    distance: u32,
    required_door_keys: KeySet,
}

fn compute_reachable(grid: &Grid, pos: Point) -> Vec<ReachableKey> {
    let key = grid_get(grid, pos).floor_key();
    let start = ReachableKey {
        pos,
        floor_key: key,
        distance: 0,
        required_door_keys: KeySet::default(),
    };

    let successors = |node: &ReachableKey, explored: &mut HashSet<Point>| {
        let mut successors = Vec::new();
        for (cell, pos) in grid_get_non_wall_neighbors(grid, node.pos) {
            if !explored.insert(pos) {
                continue; // already explored
            }
            let key = cell.floor_key();
            let required_keys = node.required_door_keys.union(cell.door_key());
            successors.push(ReachableKey {
                pos,
                floor_key: key,
                distance: node.distance + 1,
                required_door_keys: required_keys,
            })
        }
        successors
    };

    let mut queue = VecDeque::new();
    let mut explored = HashSet::new();
    let mut reachable = Vec::new();
    explored.insert(pos);
    for node in successors(&start, &mut explored) {
        queue.push_back(node);
    }
    while let Some(node) = queue.pop_front() {
        if !node.floor_key.is_empty() {
            reachable.push(node);
        }
        let successors = successors(&node, &mut explored);
        for succ in successors {
            queue.push_back(succ);
        }
    }

    reachable
}

struct Node {
    cell: Cell,
    reachable: Vec<ReachableKey>,
}

#[derive(Default)]
struct Graph {
    nodes: BTreeMap<Point, Node>,
}

impl Graph {
    fn entry_points(&self) -> Vec<Point> {
        self.nodes
            .iter()
            .filter_map(|(pos, node)| match node.cell {
                Cell::Entrance => Some(*pos),
                _ => None,
            })
            .collect()
    }

    fn num_keys(&self) -> u32 {
        self.nodes
            .values()
            .filter(|node| matches!(node.cell, Cell::Key(_)))
            .count()
            .try_into()
            .expect("key count fits in u32")
    }
}

fn compute_graph(grid: &Grid) -> Graph {
    let mut graph: Graph = Graph::default();

    let mut queue = VecDeque::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let pos = Point::new(x.try_into().unwrap(), y.try_into().unwrap());
            if matches!(cell, Cell::Entrance) {
                let reachable = compute_reachable(grid, pos);
                for node in &reachable {
                    queue.push_back(*node);
                }
                graph.nodes.insert(pos, Node { cell, reachable });
            }
        }
    }

    while let Some(node) = queue.pop_front() {
        let reachable = compute_reachable(grid, node.pos);
        for node in &reachable {
            if !graph.nodes.contains_key(&node.pos) {
                queue.push_back(*node);
            }
        }
        graph.nodes.insert(
            node.pos,
            Node {
                cell: Cell::Key(node.floor_key),
                reachable,
            },
        );
    }

    graph
}

fn parse_graph(input: &str) -> Graph {
    let trace = false;
    if trace {
        println!("input:\n{}", input);
    }
    let grid = parse_grid(input);
    let num_keys = grid
        .iter()
        .flatten()
        .filter(|ch| matches!(ch, Cell::Key(_)))
        .count();
    if false {
        println!("num_keys {:?}", num_keys);
    }

    let graph = compute_graph(&grid);

    if trace {
        println!("as graph:");
        for (pos, node) in &graph.nodes {
            println!("   pos: {} cell: {:?}", pos, node.cell);
            for reachable in &node.reachable {
                println!("      {:?}", reachable);
            }
        }
    }

    graph
}

fn solve_graph(graph: &Graph) -> u32 {
    let entry_points = graph.entry_points();
    let start = Agent {
        points: entry_points,
        keys: KeySet::default(),
    };

    let num_keys = graph.num_keys();

    let copy_and_change_point = |positions: &Vec<Point>, index: usize, pos: Point| {
        positions
            .iter()
            .enumerate()
            .map(|(i, &p)| if i == index { pos } else { p })
            .collect()
    };

    let mut count = 0;
    let successors = |agent: &Agent| -> Vec<(Agent, u32)> {
        count += 1;
        if true && count % 100_000 == 0 {
            println!("[{}] successors of {}", count, agent);
        }
        let mut successors = Vec::new();
        for (agent_index, &agent_pos) in agent.points.iter().enumerate() {
            let node = graph.nodes.get(&agent_pos).unwrap();
            for reachable in node.reachable.iter() {
                if !agent.keys.contains(reachable.required_door_keys)
                    || agent.keys.contains(reachable.floor_key)
                {
                    continue;
                }
                let keys = agent.keys.union(reachable.floor_key);
                let positions = copy_and_change_point(&agent.points, agent_index, reachable.pos);
                let successor_agent = Agent {
                    points: positions,
                    keys,
                };
                let successor_distance = reachable.distance;
                successors.push((successor_agent, successor_distance));
            }
        }
        successors
    };

    let success = |character: &Agent| character.keys.len() == num_keys;

    let path_len = {
        let (path, path_len) = dijkstra(&start, successors, success).unwrap();
        if false {
            for character in path {
                println!("path = {}", character)
            }
        }
        path_len
    };
    if true {
        println!(
            "path length: {:?}\ntook {} computed successors",
            path_len, count
        );
    }

    path_len
}

fn part_one(input: &str) -> u32 {
    let graph = parse_graph(input);
    solve_graph(&graph)
}

// Return the grid as a vector of (Point, Cell). This simplifies some code
// of performance.  When rust stabilizes generators they will be preferable
// in situations like this.  It is also possible to return an iterator, but
// the code is much more complex.  See:
// https://stackoverflow.com/a/30685840/2442218
fn grid_points(grid: &Grid) -> Vec<(Point, Cell)> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, cell)| {
                (
                    Point::new(y.try_into().unwrap(), y.try_into().unwrap()),
                    *cell,
                )
            })
        })
        .collect()
}

fn fix_for_part_two(grid: &mut Grid) {
    let entrances: Vec<Point> = grid_points(grid)
        .iter()
        .filter_map(|(pos, cell)| {
            if matches!(*cell, Cell::Entrance) {
                Some(*pos)
            } else {
                None
            }
        })
        .collect();

    // Do the transform only if there is a single entrance.
    if entrances.len() == 1 {
        let entry_point = entrances[0];

        let mut set = |x: i32, y: i32, cell: Cell| {
            let x: usize = x.try_into().unwrap();
            let y: usize = y.try_into().unwrap();
            grid[y][x] = cell;
        };

        let x = entry_point.x;
        let y = entry_point.y;

        set(y - 1, x - 1, Cell::Entrance);
        set(y - 1, x, Cell::Wall);
        set(y - 1, x + 1, Cell::Entrance);
        set(y, x - 1, Cell::Wall);
        set(y, x, Cell::Wall);
        set(y, x + 1, Cell::Wall);
        set(y + 1, x - 1, Cell::Entrance);
        set(y + 1, x, Cell::Wall);
        set(y + 1, x + 1, Cell::Entrance);
    }
}

fn part_two(input: &str) -> u32 {
    let mut grid = parse_grid(input);
    fix_for_part_two(&mut grid);
    let graph = compute_graph(&grid);
    solve_graph(&graph)
}

fn run_part_one() {
    assert_eq!(part_one(INPUT), 4770);
}

fn run_part_two() {
    assert_eq!(part_two(INPUT), 1578);
}

fn main() {
    run_part_one();
    run_part_two();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        assert_eq!(132, part_one(include_str!("../examples/18a.txt")));
    }

    #[test]
    fn test_part_one_b() {
        assert_eq!(136, part_one(include_str!("../examples/18b.txt")));
    }

    #[test]
    fn test_part_one_c() {
        assert_eq!(81, part_one(include_str!("../examples/18c.txt")));
    }

    #[test]
    fn test_part_two_a() {
        assert_eq!(8, part_two(include_str!("../examples/18d.txt")));
    }

    #[test]
    fn test_part_two_b() {
        assert_eq!(24, part_two(include_str!("../examples/18e.txt")));
    }

    #[test]
    fn test_part_two_c() {
        assert_eq!(32, part_two(include_str!("../examples/18f.txt")));
    }

    #[test]
    fn test_part_two_d() {
        assert_eq!(72, part_two(include_str!("../examples/18g.txt")));
    }

    #[test]
    fn test_part_one() {
        run_part_one();
    }

    #[test]
    fn test_part_two() {
        run_part_two();
    }
}
