pub fn parse_input(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let numbers = parse_input(input);
    let sum: u32 = numbers.iter().map(|&num| (num / 3 - 2) as u32).sum();
    Some(sum)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
