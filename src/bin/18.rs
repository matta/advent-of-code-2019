#![allow(dead_code, unused_variables)]

use std::collections::BTreeSet;

use aoc2019::point::Point2D;
use pathfinding::prelude::astar;

const INPUT: &str = include_str!("../inputs/18.txt");

type Point = Point2D<i32>;
type Grid = Vec<Vec<char>>;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Character {
    pos: Point,
    keys: BTreeSet<char>,
}

impl Character {
    fn new() -> Character {
        Character::default()
    }
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
        if count % 100000 == 0 {
            println!("successors of {:?}", character);
        }
        let succ = directions
            .iter()
            .map(|pos| *pos + character.pos)
            .filter_map(|pos| -> Option<Character> {
                match grid_get(&grid, pos) {
                    Some(ch @ 'a'..='z') => {
                        let mut keys = character.keys.clone();
                        keys.insert(ch.to_ascii_uppercase());
                        Some(Character { pos, keys })
                    }
                    Some(ch) if ch == '.' || character.keys.contains(&ch) => Some(Character {
                        pos,
                        keys: character.keys.clone(),
                    }),
                    _ => None,
                }
            })
            .map(|character| (character, 1))
            .collect::<Vec<_>>();
        // println!("successors of {:?} are {:?}", character, succ);
        succ
    };
    let heuristic = |character: &Character| num_keys - character.keys.len();
    let success = |character: &Character| character.keys.len() == num_keys;

    let result = astar(&start, successors, heuristic, success).unwrap();
    println!("result: {:?}", result);

    result.1
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
