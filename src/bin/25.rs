use std::{
    collections::{BTreeMap, BTreeSet},
    io,
};

use aoc2019::{
    graph::{Graph, NodeId},
    intcode::{Computer, RunState},
    point::CardinalDirection,
};
use itertools::Itertools;
use pathfinding::prelude::bfs;
use regex::Regex;

const INTCODE_PROGRAM: &str = include_str!("../inputs/25.txt");

fn readline() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed reading stdin");
    input
}

#[derive(Debug, Default)]
struct RoomContents {
    name: String,
    doors: Vec<CardinalDirection>,
    items: Vec<String>,
    ejected_back: bool,
}

fn parse_output(output: &str) -> Vec<RoomContents> {
    // println!("BEGIN_OUTPUT:\n{}\nEND_OUTPUT", output);
    let mut ret = Vec::new();
    let mut room = RoomContents::default();
    for line in output.lines() {
        if let Some(thing) = line.strip_prefix("- ") {
            match thing {
                "north" => room.doors.push(CardinalDirection::North),
                "south" => room.doors.push(CardinalDirection::South),
                "west" => room.doors.push(CardinalDirection::West),
                "east" => room.doors.push(CardinalDirection::East),
                thing => room.items.push(thing.to_string()),
            }
        } else if let Some(n) = line.strip_prefix("== ") {
            if let Some(n) = n.strip_suffix(" ==") {
                if !room.name.is_empty() {
                    ret.push(room);
                    room = RoomContents::default();
                }
                room.name = n.to_string();
            }
        } else if line.contains("you are ejected back") {
            room.ejected_back = true;
        } else if line.contains("You can't go that way.") {
            panic!("Bug in program.");
        }
    }
    if room.name.is_empty() {
        panic!("No named parsed in output: <<<{}>>>", output);
    }
    ret.push(room);
    ret
}

type DroidGraph = Graph<(), CardinalDirection>;

fn compute_next_move(
    graph: &DroidGraph,
    node: NodeId,
    target_node: NodeId,
) -> (CardinalDirection, NodeId) {
    println!("computing path to {:?}", target_node);
    let path = bfs(
        &None,
        |nn| {
            let id = match *nn {
                None => node,
                Some((_, id)) => id,
            };
            // println!("BFS: successors of {:?} (aka {:?})", nn, id);
            graph.successors(id).map(|(&dir, id)| Some((dir, id)))
        },
        |nn| {
            if let Some((_, id)) = *nn {
                id == target_node
            } else {
                false
            }
        },
    )
    .expect("shortest path exists");
    println!("path is: {:?}", path);

    path.iter()
        .find_map(|nn| if nn.is_some() { *nn } else { None })
        .unwrap()
}

type Movement = (NodeId, CardinalDirection);

struct PartOneDroid {
    computer: Computer,
    nodes: BTreeMap<String, NodeId>,
    graph: Graph<(), CardinalDirection>,
    unexplored: Vec<Movement>,
    from: Option<Movement>,
    security_checkpoints: BTreeSet<NodeId>,
    inventory: BTreeSet<String>,
    leave_items: bool,
}

fn print_output(output: &str) {
    for line in output.lines() {
        println!("| {}", line);
    }
}

impl PartOneDroid {
    fn new() -> Self {
        Self {
            computer: Computer::parse(INTCODE_PROGRAM),
            nodes: BTreeMap::new(),
            graph: Graph::new(),
            unexplored: Vec::new(),
            from: None,
            security_checkpoints: BTreeSet::new(),
            inventory: BTreeSet::new(),
            leave_items: false,
        }
    }

    fn add_edge(&mut self, from: NodeId, to: NodeId, dir: CardinalDirection) {
        // Avoid adding duplicate edges.
        for (&d, id) in self.graph.successors(from) {
            assert_eq!(dir == d, to == id);
            if id == to {
                return;
            }
        }
        self.graph.add_edge(from, to, dir);
    }

    fn issue_command(&mut self, command: &str) -> String {
        println!("Issue command: {}", command);
        self.computer.append_str(command);
        match self.computer.run() {
            state @ (RunState::BlockedOnInput | RunState::Finished) => {
                unreachable!("unexpected run state after issue_command: {:?}", state)
            }
            RunState::BlockedOnOutput => {
                let output = self.computer.read_ascii_string().unwrap();
                print_output(&output);
                output
            }
        }
    }

    fn issue_walk(&mut self, dir: CardinalDirection) -> String {
        let cmd = match dir {
            CardinalDirection::North => "north\n",
            CardinalDirection::South => "south\n",
            CardinalDirection::East => "east\n",
            CardinalDirection::West => "west\n",
        };
        self.issue_command(cmd)
    }

    fn take(&mut self, item: &str) {
        let forbidden = [
            "giant electromagnet",
            "photons",
            "escape pod",
            "infinite loop",
            "molten lava",
        ];
        if forbidden.contains(&item) {
            return;
        }
        let mut taken = false;
        let output = self.issue_command(&format!("take {}\n", item));
        for line in output.trim().lines() {
            if let Some(what) = line
                .strip_prefix("You take the ")
                .and_then(|rest| rest.strip_suffix('.'))
            {
                assert_eq!(what, item);
                taken = true;
            }
        }
        assert!(taken);
        self.inventory.insert(item.to_string());
    }

    fn drop(&mut self, item: &str) {
        let mut dropped = false;
        let output = self.issue_command(&format!("drop {}\n", item));
        for line in output.trim().lines() {
            if let Some(what) = line
                .strip_prefix("You drop the ")
                .and_then(|rest| rest.strip_suffix('.'))
            {
                assert_eq!(what, item);
                dropped = true;
            }
        }
        assert!(dropped);
        self.inventory.remove(item);
    }

    fn handle_output(&mut self, str: &str) -> Option<i32> {
        let contents = parse_output(str);
        println!("{:?}", contents);

        for contents in contents {
            let current_node_id = *self
                .nodes
                .entry(contents.name)
                .or_insert_with(|| self.graph.add_node(()));

            if contents.ejected_back {
                self.security_checkpoints.insert(current_node_id);
            }
            if let Some((from_node, from_dir)) = self.from {
                self.add_edge(from_node, current_node_id, from_dir);
                self.add_edge(current_node_id, from_node, from_dir.negate());
                self.from = None;
            }

            let explored_dirs: Vec<CardinalDirection> = self
                .graph
                .successors(current_node_id)
                .map(|(dir, _)| *dir)
                .collect();

            if contents.ejected_back {
                continue;
            }

            // Greedily pick everything up as soon as we see it, unless
            // we're cracking security
            if !self.leave_items {
                for item in contents.items {
                    self.take(&item);
                }
            }

            for dir in contents.doors.iter().copied() {
                if !explored_dirs.contains(&dir)
                    && !self.unexplored.contains(&(current_node_id, dir))
                {
                    println!("Push as unexplored: {:?}", (current_node_id, dir));
                    self.unexplored.push((current_node_id, dir));
                    continue;
                }
            }

            println!("unexplored: {:?}", self.unexplored);
            println!("at: {:?}", current_node_id);

            let dir = if let Some(&(target_node, target_dir)) = self.unexplored.last() {
                if target_node == current_node_id {
                    // We are already there so just move in the desired
                    // direction.
                    self.unexplored.pop();
                    target_dir
                } else {
                    let (dir, id) = compute_next_move(&self.graph, current_node_id, target_node);
                    if id == target_node {
                        self.unexplored.pop();
                    }
                    dir
                }
            } else {
                assert_eq!(1, self.security_checkpoints.len());
                let security_id = *self.security_checkpoints.first().unwrap();
                let (dir, id) = compute_next_move(&self.graph, current_node_id, security_id);
                if id == security_id {
                    // Exploration is done and we've reached the room adjacent
                    // to the pressure plate. Hack it and get the code.
                    return Some(self.hack_pressure_plate(current_node_id, dir, security_id));
                }
                dir
            };
            let cmd = match dir {
                CardinalDirection::North => "north\n",
                CardinalDirection::South => "south\n",
                CardinalDirection::East => "east\n",
                CardinalDirection::West => "west\n",
            };
            println!("Issue command: {}", cmd);
            self.computer.append_str(cmd);
            self.from = Some((current_node_id, dir))
        }
        None
    }

    fn hack_pressure_plate(
        &mut self,
        _current_node_id: NodeId,
        dir: CardinalDirection,
        _security_id: NodeId,
    ) -> i32 {
        let code_re = Regex::new("by typing (\\d+) on the keypad").unwrap();

        for permutation in self.inventory.clone().into_iter().powerset() {
            let permutation = BTreeSet::from_iter(permutation);

            let drop_items = self
                .inventory
                .difference(&permutation)
                .cloned()
                .collect_vec();
            for drop in drop_items {
                self.drop(&drop);
            }

            let take_items = permutation
                .difference(&self.inventory)
                .cloned()
                .collect_vec();
            for take in take_items {
                self.take(&take);
            }

            let output = self.issue_walk(dir);
            if output.contains("you are ejected back to the checkpoint") {
                continue;
            }

            let captures = code_re
                .captures(&output)
                .expect("code must be presented when we pass the pressure plate test");
            return captures[1].parse().unwrap();
        }
        panic!("failed to hack pressure plate")
    }
}

fn part_one() -> i32 {
    let mut this = PartOneDroid::new();

    loop {
        match this.computer.run() {
            RunState::BlockedOnInput => {
                this.computer.append_str(&readline());
            }
            RunState::BlockedOnOutput => {
                if let Some(str) = this.computer.read_ascii_string() {
                    print_output(&str);
                    if let Some(code) = this.handle_output(&str) {
                        return code;
                    }
                } else {
                    unreachable!("Non-ASCII output emitted by computer, unexpectedly!");
                }
            }
            RunState::Finished => {
                panic!("Computer finished unexpectedly.");
            }
        }
    }
}

fn main() {
    assert_eq!(part_one(), 16410);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
