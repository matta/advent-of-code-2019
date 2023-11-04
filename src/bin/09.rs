use advent_of_code::intcode::Computer;
use std::collections::VecDeque;

fn run_program(program_text: &str, input_number: i64) -> i64 {
    let mut computer = Computer::parse(program_text);
    let mut input = VecDeque::from(vec![input_number]);
    computer.run(&mut input).unwrap()
}

fn part_one(input: &str) -> i64 {
    let input_number = 1;
    run_program(input, input_number)
}

fn part_two(input: &str) -> i64 {
    let input_number = 2;
    run_program(input, input_number)
}

fn main() {
    let input = include_str!("../inputs/09.txt");
    let one = part_one(input);
    assert_eq!(one, 3906448201);
    let two = part_two(input);
    assert_eq!(two, 59785);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
