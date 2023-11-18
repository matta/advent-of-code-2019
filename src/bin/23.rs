use std::collections::VecDeque;

use aoc2019::intcode::{Computer, RunState};

const INTCODE_PROGRAM: &str = include_str!("../inputs/23.txt");
const COMPUTER_COUNT: usize = 50;

struct Node {
    computer: Computer,
    active: bool,
    packet_underflow: bool,
}

fn compute_part_one() -> i64 {
    let computer_range = 0..COMPUTER_COUNT;
    let computer = Computer::parse(INTCODE_PROGRAM);
    let mut nodes: Vec<Node> = computer_range
        .clone()
        .map(|i| {
            let mut computer = computer.clone();
            computer.append_input(&[i.try_into().unwrap()]);
            Node {
                computer,
                active: true,
                packet_underflow: false,
            }
        })
        .collect();

    let mut active: VecDeque<usize> = computer_range.clone().collect();

    while let Some(i) = active.pop_front() {
        nodes[i].active = false;
        match nodes[i].computer.run() {
            RunState::BlockedOnInput => {
                println!("{} blocked on input", i);
                let node = &mut nodes[i];
                node.computer.append_input(&[-1]);
                node.packet_underflow = true;
                node.active = true;
                active.push_back(i);
            }
            RunState::BlockedOnOutput => {
                let output = nodes[i].computer.read_output();
                assert!(output.len() % 3 == 0);
                for packet in output.chunks_exact(3) {
                    println!("{} emits {:?}", i, packet);
                    let (dest, x, y) = (packet[0] as usize, packet[1], packet[2]);
                    if dest == 255 {
                        return y;
                    }
                    let node = &mut nodes[dest];
                    node.computer.append_input(&[x, y]);
                    node.packet_underflow = false;
                    if !node.active {
                        node.active = true;
                        active.push_back(dest);
                    }
                }
            }
            RunState::Finished => {
                panic!("Computer {} finished unexpectedly.", i);
            }
        }
    }

    todo!("finished?")
}

fn part_one() {
    assert_eq!(compute_part_one(), 22659);
}

fn main() {
    part_one();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        part_one();
    }
}
