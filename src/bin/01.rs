pub(crate) fn parse_input(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input);
    let sum = input.iter().map(|&num| (num / 3 - 2)).sum();
    Some(sum)
}

pub fn fuel_part_two(input: u32) -> u32 {
    let fuel: u32 = input / 3 - 2;
    if fuel <= 5 {
        fuel
    } else {
        fuel + fuel_part_two(fuel)
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse_input(input);
    let sum = input.iter().map(|&num| fuel_part_two(num)).sum();
    Some(sum)
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
