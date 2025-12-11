use std::collections::HashMap;

use aoc2019::graph::{Graph, NodeId};
use aoc2019::point::{CardinalDirection, Point2D, CARDINAL_DIRECTIONS};
use pathfinding::prelude::{dijkstra, dijkstra_all};

type Point = Point2D<i32>;

#[derive(Debug, Clone, Copy)]
struct Edge {
    distance: u32,
    height_change: i32,
}

#[derive(Debug, Clone, Copy)]
struct Label {
    outer: bool,
    name: (char, char),
}

fn read_label(chars: &[Vec<char>], anchor: &Point, dir: CardinalDirection) -> Option<Label> {
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
            let outer = p2.x == 0 || p2.x == max_x || p2.y == 0 || p2.y == max_y;
            Some(Label {
                outer,
                name: (ch1, ch2),
            })
        }
        _ => None,
    }
}

type MazeGraph = Graph<(), Edge>;

struct Maze {
    start_id: NodeId,
    end_id: NodeId,
    graph: MazeGraph,
}

impl Maze {
    #[allow(dead_code)]
    fn print_all(&self) {
        println!("Start: {:?}", self.start_id);
        println!("End: {:?}", self.end_id);
        for (node_id, node) in self.graph.nodes() {
            println!("Node[{:?}]: {:?}", node_id, node);
            for (edge, node_id) in self.graph.successors(node_id) {
                println!("\t\t\t\t{:?} {:?}", edge, node_id);
            }
        }
    }
}

fn parse_maze(input: &str) -> Maze {
    let mut graph: Graph<(), Edge> = Graph::default();

    let chars: Vec<Vec<char>> = input
        .trim_end()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let find_attached_label = |point| {
        for dir in CARDINAL_DIRECTIONS.iter() {
            if let Some(label) = read_label(&chars, &point, *dir) {
                return Some(label);
            }
        }
        None
    };

    #[derive(Default, Debug)]
    struct Jumps {
        outer_node: Option<NodeId>,
        inner_node: Option<NodeId>,
    }
    let mut labels: HashMap<(char, char), Jumps> = HashMap::new();
    let mut nodes: HashMap<Point, NodeId> = HashMap::new();

    for (y, line) in chars.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            match ch {
                ' ' | '#' => {}
                '.' => {
                    let point = Point::usize_new(x, y).unwrap();
                    let node_id = graph.add_node(());
                    nodes.insert(point, node_id);
                    if let Some(label) = find_attached_label(point) {
                        let e = labels.entry(label.name).or_default();
                        if label.outer {
                            e.outer_node = Some(node_id);
                        } else {
                            e.inner_node = Some(node_id);
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

    for (point, node_id) in nodes.iter() {
        for neighbor_pos in point.cardinal_neighbors() {
            if let Some(neighbor_id) = nodes.get(&neighbor_pos) {
                graph.add_edge(
                    *node_id,
                    *neighbor_id,
                    Edge {
                        distance: 1,
                        height_change: 0,
                    },
                );
            }
        }
    }

    let mut start_id = None;
    let mut end_id = None;
    labels.iter().for_each(|kv| match kv {
        (
            ('A', 'A'),
            Jumps {
                outer_node: Some(id),
                inner_node: None,
            },
        ) => {
            start_id = Some(*id);
        }
        (
            ('Z', 'Z'),
            Jumps {
                outer_node: Some(id),
                inner_node: None,
            },
        ) => {
            end_id = Some(*id);
        }
        (
            _name,
            Jumps {
                outer_node: Some(outer_id),
                inner_node: Some(inner_id),
            },
        ) => {
            graph.add_edge(
                *outer_id,
                *inner_id,
                Edge {
                    distance: 1,
                    height_change: -1,
                },
            );
            graph.add_edge(
                *inner_id,
                *outer_id,
                Edge {
                    distance: 1,
                    height_change: 1,
                },
            );
        }
        _ => unreachable!("Unexpected label: {:?}", kv),
    });

    Maze {
        start_id: start_id.unwrap(),
        end_id: end_id.unwrap(),
        graph,
    }
}

fn compress_maze(original_maze: &Maze) -> Maze {
    let original_graph = &original_maze.graph;
    let mut compressed_graph = MazeGraph::new();

    // Map NodeId from the original to the compressed graph.
    let translate_map = {
        let mut translate_map = HashMap::new();
        let mut translate = |graph: &mut MazeGraph, node_id| {
            *translate_map
                .entry(node_id)
                .or_insert_with(|| graph.add_node(()))
        };

        translate(&mut compressed_graph, original_maze.start_id);
        translate(&mut compressed_graph, original_maze.end_id);
        for (source_id, _source_node) in original_graph.nodes() {
            for (edge, dest_id) in original_graph.successors(source_id) {
                if edge.height_change != 0 {
                    let source = translate(&mut compressed_graph, source_id);
                    let dest = translate(&mut compressed_graph, dest_id);
                    compressed_graph.add_edge(source, dest, *edge);
                }
            }
        }
        translate_map
    };

    // Consider only those successors that don't traverse a portal.
    let successors = |id: &NodeId| -> Vec<(NodeId, u32)> {
        original_graph
            .successors(*id)
            .filter(|(edge, _node_id)| edge.height_change == 0)
            .map(|(edge, node_id)| (node_id, edge.distance))
            .collect()
    };

    for (original_from_id, compressed_from_id) in translate_map.iter() {
        let reachable = dijkstra_all(original_from_id, successors);
        for (original_to_id, (_, distance)) in reachable.iter() {
            if let Some(compressed_to_id) = translate_map.get(original_to_id) {
                let edge = Edge {
                    distance: *distance,
                    height_change: 0,
                };
                compressed_graph.add_edge(*compressed_from_id, *compressed_to_id, edge);
            }
        }
    }

    Maze {
        start_id: *translate_map.get(&original_maze.start_id).unwrap(),
        end_id: *translate_map.get(&original_maze.end_id).unwrap(),
        graph: compressed_graph,
    }
}

fn shortest_path_part_one(maze: &Maze) -> u32 {
    let start = maze.start_id;
    let successors = |id: &NodeId| -> Vec<(NodeId, u32)> {
        maze.graph
            .successors(*id)
            .map(|(edge, node_id)| (node_id, edge.distance))
            .collect()
    };
    let success = |id: &NodeId| *id == maze.end_id;
    if let Some((_path, distance)) = dijkstra(&start, successors, success) {
        distance
    } else {
        unreachable!("no shortest path found");
    }
}

fn shortest_path_part_two(maze: &Maze) -> u32 {
    let start = (0, maze.start_id);
    let end = (0, maze.end_id);
    type PartTwoNode = (i32, NodeId);
    let successors = |(z, node_id): &PartTwoNode| {
        maze
            .graph
            .successors(*node_id)
            .filter_map(|(edge, successor_node_id)| {
                let z = z + edge.height_change;
                if z >= 0 {
                    Some(((z, successor_node_id), edge.distance))
                } else {
                    None
                }
            })
            .collect::<Vec<(PartTwoNode, u32)>>()
    };
    let success = |n: &PartTwoNode| *n == end;
    if let Some((_path, distance)) = dijkstra(&start, successors, success) {
        distance
    } else {
        unreachable!("No shortest path found!");
    }
}

pub fn main() {
    const EXAMPLE_SMALL: &str = include_str!("../examples/20.small.txt");
    let maze = parse_maze(EXAMPLE_SMALL);
    let maze = compress_maze(&maze);
    let distance = shortest_path_part_one(&maze);
    let distance2 = shortest_path_part_two(&maze);
    assert_eq!(distance, distance2);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../inputs/20.txt");
    const EXAMPLE_SMALL: &str = include_str!("../examples/20.small.txt");
    const EXAMPLE: &str = include_str!("../examples/20.txt");
    const EXAMPLE_INTERESTING: &str = include_str!("../examples/20.interesting.txt");

    #[test]
    fn test_part_one_small_example() {
        let maze = parse_maze(EXAMPLE_SMALL);
        let distance = shortest_path_part_one(&maze);
        assert_eq!(distance, 23);
    }

    #[test]
    fn test_part_one_small_example_compressed() {
        let maze = compress_maze(&parse_maze(EXAMPLE_SMALL));
        let distance = shortest_path_part_one(&maze);
        assert_eq!(distance, 23);
    }

    #[test]
    fn test_part_one_example() {
        let maze = parse_maze(EXAMPLE);
        let distance = shortest_path_part_one(&maze);
        assert_eq!(distance, 58);
    }

    #[test]
    fn test_part_one_example_compressed() {
        let maze = compress_maze(&parse_maze(EXAMPLE));
        let distance = shortest_path_part_one(&maze);
        assert_eq!(distance, 58);
    }

    #[test]
    fn test_part_one() {
        let maze = parse_maze(INPUT);
        let distance = shortest_path_part_one(&maze);
        assert_eq!(distance, 510);
    }

    #[test]
    fn test_part_one_compressed() {
        let maze = compress_maze(&parse_maze(INPUT));
        let distance = shortest_path_part_one(&maze);
        assert_eq!(distance, 510);
    }

    #[test]
    fn test_part_two_small_example() {
        let maze = compress_maze(&parse_maze(EXAMPLE_SMALL));
        let distance = shortest_path_part_two(&maze);
        assert_eq!(distance, 26);
    }

    #[test]
    fn test_part_two_interesting_example() {
        let maze = compress_maze(&parse_maze(EXAMPLE_INTERESTING));
        let distance = shortest_path_part_two(&maze);
        assert_eq!(distance, 396);
    }

    #[test]
    fn test_part_two() {
        let maze = compress_maze(&parse_maze(INPUT));
        let distance = shortest_path_part_two(&maze);
        assert_eq!(distance, 5652);
    }
}
