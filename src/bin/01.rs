pub(crate) fn parse_input(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part_one(input: &str) -> u32 {
    let input = parse_input(input);
    let sum = input
        .iter()
        .map(|&num| if num >= 9 { num / 3 - 2 } else { 0 })
        .sum();
    sum
}

fn calculate_fuel(mass: u32) -> u32 {
    let mut remaining_mass = mass;
    let mut total_fuel = 0;

    while remaining_mass >= 9 {
        let fuel = remaining_mass / 3 - 2;
        total_fuel += fuel;
        remaining_mass = fuel;
    }

    total_fuel
}

fn part_two(input: &str) -> u32 {
    let input = parse_input(input);
    let sum = input.iter().map(|&num| calculate_fuel(num)).sum();
    sum
}

fn main() {
    let input = include_str!("../inputs/01.txt").trim();
    let one = part_one(input);
    let two = part_two(input);
    assert_eq!(one, 3456641);
    assert_eq!(two, 5182078);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("0"), 0);
        assert_eq!(part_one("8"), 0);
        assert_eq!(part_one("9"), 1);
        assert_eq!(part_one("12"), 2);
        assert_eq!(part_one("14"), 2);
        assert_eq!(part_one("1969"), 654);
        assert_eq!(part_one("100756"), 33583);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("0"), 0);
        assert_eq!(part_two("8"), 0);
        assert_eq!(part_two("9"), 1);
        assert_eq!(part_two("14"), 2);
        assert_eq!(part_two("1969"), 966);
        assert_eq!(part_two("100756"), 50346);
    }

    #[test]
    fn test_main() {
        main()
    }
}
