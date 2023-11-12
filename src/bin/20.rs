#![allow(dead_code, unused_variables)]
use std::collections::HashMap;

use aoc2019::point::{CardinalDirection, Point2D, CARDINAL_DIRECTIONS};

const INPUT: &str = include_str!("../inputs/20.txt");
const EXAMPLE_SMALL: &str = include_str!("../examples/20.small.txt");
const EXAMPLE: &str = include_str!("../examples/20.txt");

type Point = Point2D<i32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Position {
    OuterLabel(char, char),
    InnerLabel(char, char),
    Corridor(Point),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    weight: u32,
    dest: Position,
}

#[derive(Default, Debug)]
struct Graph {
    nodes: HashMap<Position, Vec<Edge>>,
}

impl Graph {
    fn contains(&self, pos: &Position) -> bool {
        self.nodes.contains_key(pos)
    }

    fn print_nodes(&self) {
        let mut ordered: Vec<(&Position, &Vec<Edge>)> = self.nodes.iter().collect();
        ordered.sort_unstable();

        for (pos, edges) in ordered {
            println!("{:?}", pos);
            for edge in edges {
                println!("                         {} {:?}", edge.weight, edge.dest);
            }
        }
    }

    fn add_node(&mut self, pos: Position) {
        self.nodes.entry(pos).or_default();
    }

    fn add_edge(&mut self, from: Position, to: Position, weight: u32) {
        self.nodes
            .entry(from)
            .or_default()
            .push(Edge { weight, dest: to });
    }

    fn neighbors(&self, from: &Position) -> Vec<(Position, u32)> {
        if let Some(edges) = self.nodes.get(from) {
            edges.iter().map(|edge| (edge.dest, edge.weight)).collect()
        } else {
            Vec::new()
        }
    }

    fn points(&self) -> Vec<Position> {
        self.nodes
            .keys()
            .filter(|e| matches!(e, Position::Corridor(_)))
            .copied()
            .collect()
    }
}

fn shortest_path_part_one(graph: &Graph) -> u32 {
    let start = Position::OuterLabel('A', 'A');
    let successors = |pos: &Position| graph.neighbors(pos);
    let success = |pos: &Position| *pos == Position::OuterLabel('Z', 'Z');
    if let Some((_path, distance)) = pathfinding::prelude::dijkstra(&start, successors, success) {
        distance
    } else {
        unreachable!();
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct PositionZ {
    z: u32,
    pos: Position,
}

impl PositionZ {
    fn successor(&self, dest: &Position) -> Option<PositionZ> {
        // println!("PositionZ::successor {:?} {:?}", self, dest);
        let PositionZ { mut z, pos: source } = self;
        match (*source, *dest) {
            (Position::InnerLabel(_, _), Position::OuterLabel(_, _)) => {
                z += 1;
            }
            (Position::OuterLabel(_, _), Position::InnerLabel(_, _)) => {
                if z == 0 {
                    return None;
                }
                z -= 1;
            }
            _ => {}
        };
        Some(PositionZ { z, pos: *dest })
    }
}

fn shortest_path_part_two(graph: &Graph) -> u32 {
    let start = PositionZ {
        z: 0,
        pos: Position::OuterLabel('A', 'A'),
    };
    let end = PositionZ {
        z: 0,
        pos: Position::OuterLabel('Z', 'Z'),
    };
    let successors = |from_posz: &PositionZ| {
        graph
            .neighbors(&from_posz.pos)
            .iter()
            .filter_map(|(pos, weight)| from_posz.successor(pos).map(|succ| (succ, *weight)))
            .collect::<Vec<_>>()
    };
    let success = |pos: &PositionZ| *pos == end;
    if let Some((_path, distance)) = pathfinding::prelude::dijkstra(&start, successors, success) {
        distance
    } else {
        unreachable!();
    }
}

fn read_label(chars: &[Vec<char>], anchor: &Point, dir: CardinalDirection) -> Option<Position> {
    let get = |pos: Point| -> Option<char> {
        let (x, y) = pos.as_usize_pair()?;
        Some(*chars.get(y)?.get(x)?)
    };

    let p1 = anchor.cardinal_neighbor(dir);
    let p2 = p1.cardinal_neighbor(dir);
    match (get(p1), get(p2)) {
        (Some(mut ch1), Some(mut ch2)) if ch1.is_ascii_uppercase() && ch2.is_ascii_uppercase() => {
            match dir {
                CardinalDirection::North | CardinalDirection::West => (ch1, ch2) = (ch2, ch1),
                CardinalDirection::South | CardinalDirection::East => {}
            }
            let max_x: i32 = (chars[0].len() - 1).try_into().unwrap();
            let max_y: i32 = (chars.len() - 1).try_into().unwrap();
            let label = if p2.x == 0 || p2.x == max_x || p2.y == 0 || p2.y == max_y {
                Position::OuterLabel(ch1, ch2)
            } else {
                Position::InnerLabel(ch1, ch2)
            };
            Some(label)
        }
        _ => None,
    }
}

fn parse_maze(input: &str) -> Graph {
    let mut graph = Graph::default();

    let chars: Vec<Vec<char>> = input
        .trim_end()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let find_attached_label = |point| {
        for dir in CARDINAL_DIRECTIONS.iter() {
            if let Some(label) = read_label(&chars, &point, *dir) {
                return Some((*dir, label));
            }
        }
        None
    };

    for (y, line) in chars.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            let pos = Point::usize_new(x, y).unwrap();
            match ch {
                ' ' | '#' => {}
                '.' => {
                    let point = Point::usize_new(x, y).unwrap();
                    let corridor = Position::Corridor(point);
                    graph.add_node(corridor);
                    if let Some((dir, label)) = find_attached_label(point) {
                        graph.add_edge(label, corridor, 0);
                        graph.add_edge(corridor, label, 0);
                        match label {
                            Position::OuterLabel(ch1, ch2) => {
                                graph.add_edge(label, Position::InnerLabel(ch1, ch2), 1);
                            }
                            Position::InnerLabel(ch1, ch2) => {
                                graph.add_edge(label, Position::OuterLabel(ch1, ch2), 1);
                            }
                            _ => {}
                        }
                    } else {
                        graph.add_node(corridor);
                    }
                }
                ch if ch.is_ascii_uppercase() => {
                    // Handled in read_label() above.
                }
                _ => {
                    unreachable!("unexpected input '{}", ch);
                }
            }
        }
    }

    for pos in graph.points() {
        if let Position::Corridor(point) = pos {
            for neighbor_pos in point.cardinal_neighbors() {
                let neighbor = Position::Corridor(neighbor_pos);
                if graph.contains(&neighbor) {
                    graph.add_edge(pos, neighbor, 1);
                }
            }
        } else {
            unreachable!();
        }
    }

    graph
}

pub fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one_small_example() {
        let graph = parse_maze(EXAMPLE_SMALL);
        let distance = shortest_path_part_one(&graph);
        assert_eq!(distance, 23);
    }

    #[test]
    fn test_part_one_example() {
        let graph = parse_maze(EXAMPLE);
        let distance = shortest_path_part_one(&graph);
        assert_eq!(distance, 58);
    }

    #[test]
    fn test_part_one() {
        let graph = parse_maze(INPUT);
        let distance = shortest_path_part_one(&graph);
        assert_eq!(distance, 510);
    }

    #[test]
    fn test_part_two_small_example() {
        let graph = parse_maze(EXAMPLE_SMALL);
        let distance = shortest_path_part_two(&graph);
        assert_eq!(distance, 26);
    }

    #[test]
    fn test_part_two() {
        let graph = parse_maze(INPUT);
        let distance = shortest_path_part_two(&graph);
        assert_eq!(distance, 5652);
    }
}
