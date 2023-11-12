#![allow(dead_code, unused_variables)]
use std::collections::HashMap;

use aoc2019::point::{CardinalDirection, Point2D, CARDINAL_DIRECTIONS};

const INPUT: &str = include_str!("../inputs/20.txt");
const EXAMPLE_SMALL: &str = include_str!("../examples/20.small.txt");
const EXAMPLE: &str = include_str!("../examples/20.txt");

type Point = Point2D<i32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Position {
    Label(char, char),
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

fn measure_maze(graph: &Graph) -> u32 {
    let start = Position::Label('A', 'A');
    let successors = |pos: &Position| {
        // println!("successors of {:?} => {:?}", pos, res);
        graph.neighbors(pos)
    };
    let success = |pos: &Position| *pos == Position::Label('Z', 'Z');
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
        (Some(ch1), Some(ch2)) if ch1.is_ascii_uppercase() && ch2.is_ascii_uppercase() => {
            Some(match dir {
                CardinalDirection::North | CardinalDirection::West => Position::Label(ch2, ch1),
                CardinalDirection::South | CardinalDirection::East => Position::Label(ch1, ch2),
            })
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

    for (y, line) in chars.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            let pos = Point::usize_new(x, y).unwrap();
            match ch {
                ' ' | '#' => {}
                '.' => {
                    let point = Point::usize_new(x, y).unwrap();
                    let corridor = Position::Corridor(point);
                    graph.add_node(corridor);
                    for dir in CARDINAL_DIRECTIONS.iter() {
                        if let Some(label) = read_label(&chars, &point, *dir) {
                            match label {
                                Position::Label('A', 'A') => {
                                    graph.add_edge(label, corridor, 0);
                                }
                                Position::Label('Z', 'Z') => {
                                    graph.add_edge(corridor, label, 0);
                                }
                                _ => {
                                    graph.add_edge(corridor, label, 1);
                                    graph.add_edge(label, corridor, 0);
                                }
                            }
                        }
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
        let distance = measure_maze(&graph);
        assert_eq!(distance, 23);
    }

    #[test]
    fn test_part_one_example() {
        let graph = parse_maze(EXAMPLE);
        let distance = measure_maze(&graph);
        assert_eq!(distance, 58);
    }

    #[test]
    fn test_part_one() {
        let graph = parse_maze(INPUT);
        let distance = measure_maze(&graph);
        assert_eq!(distance, 510);
    }
}
