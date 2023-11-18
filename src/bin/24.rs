use std::collections::HashSet;

const INPUT: &str = include_str!("../inputs/24.txt");

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

fn compute_part_two() -> usize {
    0
}

fn part_one() {
    assert_eq!(compute_part_one(), 18844281);
}

fn part_two() {
    assert_eq!(compute_part_two(), 0);
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
    fn test_part_two() {
        part_two();
    }
}
