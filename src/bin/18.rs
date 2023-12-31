// Code could be much cleaner.  I stopped after it worked.
//
// I optimized this until the tests passed in 0.02s, which took a multi-pass
// approach.
//
// 1) Create the graph as faithful representation of the input, with length=1
//    edges to all non-wall positions.
// 2) Remove all empty nodes from the graph, leaving only the start, key, and
//    door nodes.  Call this set of nodes N.  This is done by running a BFS
//    from each node in N until another node in N is reached, recording
//    the distance to each.
// 3) Now, call the set of start nodes S and the set of target nodes T where
//    T is N - S.  Construct a new graph for nodes N where every node has an
//    edge to T.  Each edge has a distance, the set of keys found while
//    traversing the edge, and the set of keys required to traverse it.
// 4) Run Djikstra's shortest path over the graph constructed in 3.
//
// Of these steps by far the most important is step 2.  Step 3 is an
// an optimization that is less important.
//
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::hash::Hash;
use std::time::Instant;

use aoc2019::point::Point2D;
use pathfinding::prelude::dijkstra;

const INPUT: &str = include_str!("../inputs/18.txt");

type Point = Point2D<i32>;
type Grid = Vec<Vec<Cell>>;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct KeySet {
    mask: u32,
}

impl fmt::Debug for KeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for ch in 'A'..='Z' {
            let key: KeySet = ch.try_into().unwrap();
            if self.contains(key) {
                write!(f, "{}", ch)?
            }
        }
        write!(f, "}}")
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

    fn intersection(&self, other: KeySet) -> KeySet {
        KeySet {
            mask: self.mask & other.mask,
        }
    }

    fn difference(&self, other: KeySet) -> KeySet {
        KeySet {
            mask: self.mask & !other.mask,
        }
    }

    fn contains(&self, other: KeySet) -> bool {
        self.intersection(other) == other
    }
}

impl TryFrom<u32> for KeySet {
    type Error = &'static str;

    fn try_from(mask: u32) -> Result<Self, Self::Error> {
        let max_mask = 1 << 26;
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
        let mask = 1 << offset;
        let key: KeySet = mask.try_into()?;
        Ok(key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    keys: KeySet,
    positions: Vec<EdgeNode>,
}

impl Agent {
    fn new(keys: KeySet, positions: &[EdgeNode]) -> Agent {
        Agent {
            keys,
            positions: positions.into(),
        }
    }
}

impl fmt::Display for Agent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "<{:?} ", self.positions)?;
        write!(f, "{:?}", self.keys)?;
        write!(f, ">")
    }
}

fn parse_grid(input: &str) -> Grid {
    let trace = false;
    if trace {
        println!("input:\n{}", input);
    }
    let grid: Grid = input
        .trim()
        .split_ascii_whitespace()
        .map(|line| {
            line.chars()
                .map(|ch| {
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

#[derive(Debug, Clone, Copy)]
struct ReachableKey {
    node_id: EdgeNode,
    distance: u32,
    keys: KeySet,
    required_keys: KeySet,
}

#[derive(Debug)]
struct GraphEdges {
    reachable: Vec<ReachableKey>,
}

type GraphPoint = EdgeNode;

#[derive(Default)]
struct Graph {
    nodes: BTreeMap<GraphPoint, GraphEdges>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct EdgeNode {
    pos: Point,
    cell: Cell,
}

type EdgesFromNodeVec = Vec<(EdgeNode, u32)>;
type EdgeNodeMap = BTreeMap<EdgeNode, EdgesFromNodeVec>;

impl Graph {
    fn entry_points(&self) -> Vec<GraphPoint> {
        self.nodes
            .keys()
            .filter_map(|pos| match pos.cell {
                Cell::Entrance => Some(*pos),
                _ => None,
            })
            .collect()
    }

    fn num_keys(&self) -> u32 {
        self.nodes
            .keys()
            .filter(|node| matches!(node.cell, Cell::Key(_)))
            .count()
            .try_into()
            .expect("key count fits in u32")
    }
}

#[allow(dead_code)]
fn print_nodes(edges: &EdgeNodeMap, phase: &str) {
    println!("\nEDGES for phase {}:", phase);
    for (node, node_edges) in edges.iter() {
        println!("{:?}", node);
        for edge in node_edges {
            println!("            {:?}", edge);
        }
    }
}

fn compute_all_edges(grid: &Grid) -> EdgeNodeMap {
    let mut nodes = BTreeMap::new();

    let directions: [Point; 4] = [
        Point::new(0, -1),
        Point::new(0, 1),
        Point::new(-1, 0),
        Point::new(1, 0),
    ];

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if matches!(cell, Cell::Wall) {
                continue;
            }
            let pos = Point::new(x.try_into().unwrap(), y.try_into().unwrap());
            let node = EdgeNode { pos, cell };
            let mut edges = EdgesFromNodeVec::default();

            for dest_pos in directions.iter().map(|dir| pos + *dir) {
                let dest_cell = grid_get(grid, dest_pos);
                if matches!(dest_cell, Cell::Wall) {
                    continue;
                }

                edges.push((
                    EdgeNode {
                        pos: dest_pos,
                        cell: dest_cell,
                    },
                    1,
                ));
            }

            if !edges.is_empty() {
                nodes.insert(node, edges);
            }
        }
    }

    nodes
}

fn compress_edges(nodes: &EdgeNodeMap) -> EdgeNodeMap {
    let keep_nodes: Vec<EdgeNode> = nodes
        .keys()
        .filter(|e| e.cell != Cell::Open)
        .copied()
        .collect();
    let mut compressed_nodes = EdgeNodeMap::new();
    for keep_node in keep_nodes {
        let mut edges = EdgesFromNodeVec::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((keep_node, 0_u32));
        while let Some((src, src_weight)) = queue.pop_front() {
            visited.insert(src);
            for (dst, dst_weight) in nodes.get(&src).unwrap().iter() {
                if visited.contains(dst) {
                    continue;
                }
                let weight = src_weight + dst_weight;
                if dst.cell == Cell::Open || dst.cell == Cell::Entrance {
                    queue.push_back((*dst, weight));
                } else {
                    edges.push((*dst, weight));
                }
            }
        }
        compressed_nodes.insert(keep_node, edges);
    }
    compressed_nodes
}

fn compute_edges(grid: &Grid) -> EdgeNodeMap {
    let edges = compute_all_edges(grid);
    compress_edges(&edges)
}

fn compute_reachable(edges: &EdgeNodeMap, start_node: &EdgeNode) -> Vec<ReachableKey> {
    let start = ReachableKey {
        node_id: *start_node,
        keys: start_node.cell.floor_key(),
        distance: 0,
        required_keys: KeySet::default(),
    };
    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut explored = HashSet::new();
    explored.insert(start.node_id.pos);

    let mut reachable = Vec::new();
    while let Some(node) = queue.pop_front() {
        if !node.node_id.cell.floor_key().is_empty() {
            reachable.push(node);
        }
        let node_edges = edges.get(&node.node_id).unwrap();

        for (edge_node, weight) in node_edges.iter() {
            if !explored.insert(edge_node.pos) {
                continue;
            }

            let cell = edge_node.cell;
            let floor_keys = node.keys.union(cell.floor_key());
            let required_keys = node.required_keys.union(cell.door_key());
            queue.push_back(ReachableKey {
                node_id: *edge_node,
                keys: floor_keys,
                distance: node.distance + weight,
                required_keys,
            })
        }
    }

    reachable
}

fn compute_graph(edges: &EdgeNodeMap) -> Graph {
    let mut graph: Graph = Graph::default();

    for node in edges.keys() {
        if matches!(node.cell, Cell::Entrance | Cell::Key(_)) {
            let reachable = compute_reachable(edges, node);
            graph.nodes.insert(*node, GraphEdges { reachable });
        }
    }

    let trace = false;
    if trace {
        println!("as graph:");
        for (pos, node) in &graph.nodes {
            println!("   pos: {:?}", pos);
            for reachable in &node.reachable {
                println!("      {:?}", reachable);
            }
        }
    }

    graph
}

fn solve_graph(graph: &Graph) -> u32 {
    let entry_points = graph.entry_points();
    let start = Agent::new(KeySet::default(), &entry_points);
    let num_keys = graph.num_keys();

    let mut successors_call_count = 0;
    let mut successors_count = 0;
    let successors = |agent: &Agent| -> Vec<(Agent, u32)> {
        successors_call_count += 1;
        if successors_call_count % 100_000 == 0 {
            println!("[{}] successors of {}", successors_call_count, agent);
        }
        let mut successors = Vec::new();
        for (positions_index, &position) in agent.positions.iter().enumerate() {
            let node = graph.nodes.get(&position).unwrap();
            for reachable in node.reachable.iter() {
                // Ignore positions where we lack the required keys to get there from here.
                if !agent.keys.contains(reachable.required_keys) {
                    continue;
                }

                // Minmize the state transitions to those that gain one key. If
                // we gain no keys there is no point in moving to the position. If we gain more
                // than one key there must be an alternative position that gains only one.
                let added_keys = reachable.keys.difference(agent.keys);
                if added_keys.len() != 1 {
                    continue;
                }

                let mut successor_agent = agent.clone();
                successor_agent.keys = agent.keys.union(reachable.keys);
                successor_agent.positions[positions_index] = reachable.node_id;
                let distance = reachable.distance;
                successors.push((successor_agent, distance));
            }
        }
        successors_count += successors.len();
        successors
    };

    let success = |character: &Agent| character.keys.len() == num_keys;

    let path_len = {
        let (path, path_len) = dijkstra(&start, successors, success).expect("no path found");
        if false {
            for character in path {
                println!("path = {}", character)
            }
        }
        path_len
    };
    if true {
        println!(
            "path length: {:?}\ntook {} calls to successors() and {} states",
            path_len, successors_call_count, successors_count
        );
    }

    path_len
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
                    Point::new(x.try_into().unwrap(), y.try_into().unwrap()),
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

enum Part {
    One,
    Two,
}

fn solve_part(input: &str, part: Part) -> u32 {
    let mut start = Instant::now();
    let mut grid = parse_grid(input);
    if matches!(part, Part::Two) {
        fix_for_part_two(&mut grid);
    }
    let parse_duration = start.elapsed();

    start = Instant::now();
    let nodes = compute_edges(&grid);
    let edges_duration = start.elapsed();

    start = Instant::now();
    let graph = compute_graph(&nodes);
    let graph_duration = start.elapsed();

    start = Instant::now();
    let path_length = solve_graph(&graph);
    let solve_duration = start.elapsed();

    println!(
        "parse_duration {:#?} edges_duration {:?} graph_duration {:?} solve_duration {:?}",
        parse_duration, edges_duration, graph_duration, solve_duration
    );
    path_length
}

fn part_one(input: &str) -> u32 {
    solve_part(input, Part::One)
}

fn part_two(input: &str) -> u32 {
    solve_part(input, Part::Two)
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
    fn test_key_set_default() {
        assert_eq!(KeySet::default().mask, 0);
        assert_eq!(format!("{:?}", KeySet::default()), "{}");
    }

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
