#![allow(dead_code, unused_variables)]

use std::fmt;

use aoc2019::point::Point2D;
use pathfinding::prelude::astar;

const INPUT: &str = include_str!("../inputs/18.txt");

type Point = Point2D<i32>;
type Grid = Vec<Vec<char>>;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Character {
    pos: Point,
    keys: u32, // 'a' is bit 0, 'b' is bit 1, etc.
}

impl Character {
    fn new() -> Character {
        Character::default()
    }

    fn set_key(&mut self, key: char) {
        assert!(matches!(key, 'a'..='z'));
        let mask: u32 = (1 as u32) << ((key as u32) - ('a' as u32));
        self.keys |= mask;
    }

    fn have_key(&self, key: char) -> bool {
        assert!(matches!(key, 'A'..='Z'));
        let mask: u32 = (1 as u32) << ((key as u32) - ('A' as u32));
        (self.keys & mask) != 0
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "<{} ", self.pos)?;
        for ch in 'A'..='Z' {
            let c = if self.have_key(ch) { ch } else { '.' };
            write!(f, "{}", c)?;
        }
        write!(f, ">")
    }
}

#[cfg(test)]
#[test]
fn test_character_keys() {
    let mut c = Character::new();
    assert_eq!(format!("{}" ,c), "<(0, 0) ..........................>");

    assert_eq!(false, c.have_key('A'));
    c.set_key('a');
    assert_eq!(true, c.have_key('A'));
    assert_eq!(format!("{}" ,c), "<(0, 0) A.........................>");

    assert_eq!(false, c.have_key('Z'));
    c.set_key('z');
    assert_eq!(true, c.have_key('Z'));
    assert_eq!(format!("{}" ,c), "<(0, 0) A........................Z>");
}

fn parse_input(input: &str) -> (Grid, Character) {
    let mut character = Character::new();
    let grid: Grid = input
        .trim()
        .split_ascii_whitespace()
        .enumerate()
        .map(|(y, line)| {
            let row: Vec<char> = line
                .chars()
                .enumerate()
                .map(|(x, ch)| match ch {
                    '@' => {
                        character.pos = Point::new(x.try_into().unwrap(), y.try_into().unwrap());
                        '.'
                    }
                    '#' | '.' | 'A'..='Z' | 'a'..='z' => ch,
                    _ => panic!("unexpected character '{}'", ch),
                })
                .collect();
            row
        })
        .collect();
    (grid, character)
}

fn grid_get(grid: &Grid, pos: Point) -> Option<char> {
    if pos.y < 0 && pos.x < 0 {
        return None;
    }
    grid.get(pos.y as usize)
        .and_then(|row| row.get(pos.x as usize))
        .copied()
}

fn part_one(input: &str) -> usize {
    let (grid, start) = parse_input(input);
    let num_keys = grid
        .iter()
        .flatten()
        .filter(|ch| matches!(ch, 'a'..='z'))
        .count();
    println!("start {:?}", start);
    println!("num_keys {:?}", num_keys);

    let directions: [Point; 4] = [
        Point::new(0, -1),
        Point::new(0, 1),
        Point::new(-1, 0),
        Point::new(1, 0),
    ];

    let mut count = 0;
    let successors = |character: &Character| {
        count += 1;
        if count % 100_000 == 0 {
            println!("{} successors of {}", count, character);
        }
        let succ = directions
            .iter()
            .map(|direction| *direction + character.pos)
            .filter_map(|neighbor_pos| -> Option<Character> {
                match grid_get(&grid, neighbor_pos) {
                    Some(ch @ 'a'..='z') => {
                        let mut neighbor = Character {
                            pos: neighbor_pos,
                            keys: character.keys,
                        };
                        neighbor.set_key(ch);
                        Some(neighbor)
                    }
                    Some(ch)
                        if ch == '.' || (matches!(ch, 'A'..='Z') && character.have_key(ch)) =>
                    {
                        Some(Character {
                            pos: neighbor_pos,
                            keys: character.keys,
                        })
                    }
                    _ => None,
                }
            })
            .map(|character| (character, 1))
            .collect::<Vec<_>>();
        // println!("successors of {:?} are {:?}", character, succ);
        succ
    };
    let heuristic = |character: &Character| num_keys - character.keys.count_ones() as usize;
    let success = |character: &Character| character.keys.count_ones() as usize == num_keys;

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
