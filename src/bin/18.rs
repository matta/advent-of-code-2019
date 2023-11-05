#![allow(dead_code, unused_variables)]

use std::fmt;

use aoc2019::point::Point2D;
use pathfinding::prelude::astar;

const INPUT: &str = include_str!("../inputs/18.txt");

type Point = Point2D<i32>;
type Grid = Vec<Vec<Cell>>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct KeySet {
    mask: u32,
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
    pos: Point,
    keys: u32, // 'a' is bit 0, 'b' is bit 1, etc.
}

impl Agent {
    fn new() -> Agent {
        Agent::default()
    }

    fn set_key(&mut self, key: KeySet) {
        self.keys |= key.mask;
    }

    fn have_key(&self, key: KeySet) -> bool {
        (self.keys & key.mask) != 0
    }
}

impl fmt::Display for Agent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "<{} ", self.pos)?;
        for ch in 'A'..='Z' {
            let key: KeySet = ch.try_into().unwrap();
            let c = if self.have_key(key) { ch } else { '.' };
            write!(f, "{}", c)?;
        }
        write!(f, ">")
    }
}

#[cfg(test)]
#[test]
fn test_character_keys() {
    let mut c = Agent::new();
    assert_eq!(format!("{}", c), "<(0, 0) ..........................>");

    assert_eq!(false, c.have_key('A'.try_into().unwrap()));
    c.set_key('a'.try_into().unwrap());
    assert_eq!(true, c.have_key('A'.try_into().unwrap()));
    assert_eq!(format!("{}", c), "<(0, 0) A.........................>");

    assert_eq!(false, c.have_key('Z'.try_into().unwrap()));
    c.set_key('z'.try_into().unwrap());
    assert_eq!(true, c.have_key('Z'.try_into().unwrap()));
    assert_eq!(format!("{}", c), "<(0, 0) A........................Z>");
}

fn parse_input(input: &str) -> (Grid, Agent) {
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

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if matches!(cell, Cell::Entrance) {
                return (
                    grid,
                    Agent {
                        pos: Point::new(x.try_into().unwrap(), y.try_into().unwrap()),
                        keys: 0,
                    },
                );
            }
        }
    }

    unreachable!("grid had no entrance!")
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

fn part_one(input: &str) -> usize {
    let (grid, start) = parse_input(input);
    let num_keys = grid
        .iter()
        .flatten()
        .filter(|ch| matches!(ch, Cell::Key(_)))
        .count();
    println!("start {:?}", start);
    println!("num_keys {:?}", num_keys);

    let mut count = 0;
    let successors = |agent: &Agent| {
        count += 1;
        if count % 100_000 == 0 {
            println!("{} successors of {}", count, agent);
        }
        let succ = grid_get_non_wall_neighbors(&grid, agent.pos)
            .iter()
            .copied()
            .filter_map(|(cell, neighbor_pos)| -> Option<Agent> {
                match cell {
                    Cell::Key(mask) => {
                        let mut neighbor = Agent {
                            pos: neighbor_pos,
                            keys: agent.keys,
                        };
                        neighbor.set_key(mask);
                        Some(neighbor)
                    }
                    Cell::Open | Cell::Entrance => Some(Agent {
                        pos: neighbor_pos,
                        keys: agent.keys,
                    }),
                    Cell::Door(key) if agent.have_key(key) => Some(Agent {
                        pos: neighbor_pos,
                        keys: agent.keys,
                    }),
                    Cell::Wall | Cell::Door(_) => None,
                }
            })
            .map(|character| (character, 1))
            .collect::<Vec<_>>();
        // println!("successors of {:?} are {:?}", character, succ);
        succ
    };
    let heuristic = |character: &Agent| num_keys - character.keys.count_ones() as usize;
    let success = |character: &Agent| character.keys.count_ones() as usize == num_keys;

    let (path, path_len) = astar(&start, successors, heuristic, success).unwrap();
    for character in path {
        println!("path = {}", character)
    }
    println!("path_len: {:?}", path_len);

    path_len
}

fn main() {
    assert_eq!(part_one(INPUT), 4770);
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
    fn test_main() {
        main();
    }
}
