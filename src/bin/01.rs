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

pub fn calculate_fuel(mass: u32) -> u32 {
    let mut remaining_mass = mass;
    let mut total_fuel = 0;

    while remaining_mass >= 9 {
        let fuel = remaining_mass / 3 - 2;
        total_fuel += fuel;
        remaining_mass = fuel;
    }

    total_fuel
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse_input(input);
    let sum = input.iter().map(|&num| calculate_fuel(num)).sum();
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
