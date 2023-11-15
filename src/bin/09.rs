use aoc2019::intcode::{Computer, RunState};

fn run_program(program_text: &str, input_number: i64) -> i64 {
    let mut computer = Computer::parse(program_text);
    computer.append_input(&[input_number]);
    if let RunState::BlockedOnOutput(output) = computer.run() {
        return output;
    }
    unreachable!("program never produced output");
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
