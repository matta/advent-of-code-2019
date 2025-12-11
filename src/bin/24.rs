use std::{
    collections::{BTreeMap, HashSet},
    ops::Range,
};

const INPUT: &str = include_str!("../inputs/24.txt");
const TILE_RANGE: Range<u32> = 0..5;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct Tile(u32);

impl Tile {
    fn new() -> Tile {
        Tile::default()
    }

    fn parse(input: &str) -> Tile {
        let mut tile = 0;
        for (i, ch) in input
            .chars()
            .filter(|ch| *ch == '#' || *ch == '.')
            .enumerate()
        {
            if ch == '#' {
                tile |= 1 << i;
            }
        }
        Tile(tile)
    }

    fn get(&self, x: u32, y: u32) -> bool {
        (Tile::mask(x, y) & self.0) != 0
    }

    fn mask(x: u32, y: u32) -> u32 {
        1 << (y * 5 + x)
    }

    fn count_bugs(&self) -> u32 {
        self.0.count_ones()
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in TILE_RANGE {
            for x in TILE_RANGE {
                write!(
                    f,
                    "{}",
                    match self.get(x, y) {
                        false => '.',
                        true => '#',
                    }
                )?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Eris {
    matrix: u32,
}

impl Eris {
    fn parse(input: &str) -> Eris {
        let mut matrix = 0;
        for (i, ch) in input
            .chars()
            .filter(|ch| *ch == '#' || *ch == '.')
            .enumerate()
        {
            if ch == '#' {
                matrix |= 1 << i;
            }
        }
        Eris { matrix }
    }

    fn step(&self) -> Eris {
        let mut next = self.clone();
        for y in 0..5 {
            for x in 0..5 {
                let bug_mask = self.mask(x, y);
                let count = self.count_adjacent(x, y);
                let bug = (next.matrix & bug_mask) != 0;
                if bug && count != 1 {
                    // bug dies
                    next.matrix &= !bug_mask;
                }
                if !bug && (1..=2).contains(&count) {
                    // bug is born
                    next.matrix |= bug_mask;
                }
            }
        }
        next
    }

    fn count_adjacent(&self, x: usize, y: usize) -> u32 {
        let mut count = 0;
        if y > 0 && self.is_bug(x, y - 1) {
            count += 1;
        }
        if x > 0 && self.is_bug(x - 1, y) {
            count += 1;
        }
        if x < 4 && self.is_bug(x + 1, y) {
            count += 1;
        }
        if y < 4 && self.is_bug(x, y + 1) {
            count += 1;
        }
        count
    }

    fn is_bug(&self, x: usize, y: usize) -> bool {
        (self.mask(x, y) & self.matrix) != 0
    }

    fn mask(&self, x: usize, y: usize) -> u32 {
        1 << (y * 5 + x)
    }

    fn biodiversity(&self) -> u32 {
        self.matrix
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
    fn new(x: i32, y: i32, z: i32) -> Pos {
        Pos { x, y, z }
    }

    fn north(&self) -> Pos {
        Pos::new(self.x, self.y - 1, self.z)
    }

    fn south(&self) -> Pos {
        Pos::new(self.x, self.y + 1, self.z)
    }

    fn west(&self) -> Pos {
        Pos::new(self.x - 1, self.y, self.z)
    }

    fn east(&self) -> Pos {
        Pos::new(self.x + 1, self.y, self.z)
    }
}

#[derive(Debug, Clone)]
struct RecursiveEris {
    levels: BTreeMap<i32, Tile>,
}

impl std::fmt::Display for RecursiveEris {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (level, tile) in self.levels.iter() {
            writeln!(f, "Depth: {}", level)?;
            writeln!(f, "{}", tile)?;
        }
        Ok(())
    }
}

impl RecursiveEris {
    fn parse(input: &str) -> RecursiveEris {
        let tile = Tile::parse(input);
        let mut levels = BTreeMap::new();
        levels.insert(0, tile);
        RecursiveEris { levels }
    }

    fn step(&self) -> RecursiveEris {
        let mut next = self.clone();

        // Add an empty level above and below the current levels.
        if let Some((level, tile)) = next.levels.first_key_value()
            && *tile != Tile::new() {
                next.levels.insert(level - 1, Tile::new());
            }
        if let Some((level, tile)) = next.levels.last_key_value()
            && *tile != Tile::new() {
                next.levels.insert(level + 1, Tile::new());
            }

        let keys: Vec<i32> = next.levels.keys().copied().collect();
        for z in keys.into_iter() {
            for y in 0..5 {
                for x in 0..5 {
                    if x == 2 && y == 2 {
                        continue;
                    }
                    let pos = Pos::new(x, y, z);
                    let count = self.count_adjacent(pos);
                    let bug = self.is_bug(pos);
                    // println!("{:?} count:{} bug:{}", pos, count, bug);
                    if bug && count != 1 {
                        next.clear_bug(pos);
                    }
                    if !bug && (1..=2).contains(&count) {
                        next.spawn_bug(pos);
                    }
                }
            }
        }
        next
    }

    fn count_bugs(&self) -> u32 {
        self.levels.values().map(|level| level.count_bugs()).sum()
    }

    fn count_adjacent(&self, pos: Pos) -> u32 {
        let get = |pos| {
            if self.is_bug(pos) {
                1
            } else {
                0
            }
        };
        let mut sum = 0;
        let outer_z = pos.z - 1;
        let inner_z = pos.z + 1;
        if pos.y == 0 {
            sum += get(Pos::new(2, 1, outer_z));
        } else {
            sum += get(pos.north());
        }
        if pos.y == 4 {
            sum += get(Pos::new(2, 3, outer_z));
        } else {
            sum += get(pos.south());
        }
        if pos.x == 0 {
            sum += get(Pos::new(1, 2, outer_z));
        } else {
            sum += get(pos.west());
        }
        if pos.x == 4 {
            sum += get(Pos::new(3, 2, outer_z));
        } else {
            sum += get(pos.east());
        }
        match (pos.x, pos.y) {
            (2, 1) => {
                sum += (0..5).map(|xx| get(Pos::new(xx, 0, inner_z))).sum::<u32>();
            }
            (1, 2) => {
                sum += (0..5).map(|yy| get(Pos::new(0, yy, inner_z))).sum::<u32>();
            }
            (2, 3) => {
                sum += (0..5).map(|xx| get(Pos::new(xx, 4, inner_z))).sum::<u32>();
            }
            (3, 2) => {
                sum += (0..5).map(|yy| get(Pos::new(4, yy, inner_z))).sum::<u32>();
            }
            _ => {}
        }
        sum
    }

    fn clear_bug(&mut self, pos: Pos) {
        let tile = self.levels.get_mut(&pos.z).unwrap();
        tile.0 &= !Tile::mask(pos.x.try_into().unwrap(), pos.y.try_into().unwrap());
    }

    fn spawn_bug(&mut self, pos: Pos) {
        let tile = self.levels.get_mut(&pos.z).unwrap();
        tile.0 |= Tile::mask(pos.x.try_into().unwrap(), pos.y.try_into().unwrap());
    }

    fn is_bug(&self, pos: Pos) -> bool {
        if let Some(tile) = self.levels.get(&pos.z) {
            tile.get(pos.x.try_into().unwrap(), pos.y.try_into().unwrap())
        } else {
            false
        }
    }
}

fn compute_part_one() -> u32 {
    let mut seen = HashSet::new();
    let mut eris = Eris::parse(INPUT);
    seen.insert(eris.clone());

    loop {
        let next = eris.step();
        if !seen.insert(next.clone()) {
            return next.biodiversity();
        }
        eris = next;
    }
}

fn compute_part_two(steps: u32, input: &str) -> u32 {
    let mut eris = RecursiveEris::parse(input);
    for _ in 0..steps {
        eris = eris.step();
    }
    eris.count_bugs()
}

fn part_one() {
    assert_eq!(compute_part_one(), 18844281);
}

fn part_two() {
    assert_eq!(compute_part_two(200, INPUT), 1872);
}

fn main() {
    part_one();
    part_two();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        part_one();
    }

    #[test]
    fn test_part_two_example() {
        let input = "....#\n#..#.\n#..##\n..#..\n#....";
        assert_eq!(compute_part_two(10, input), 99);
    }

    #[test]
    fn test_part_two() {
        part_two();
    }
}
