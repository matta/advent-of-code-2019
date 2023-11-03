use itertools::Itertools;
use std::collections::VecDeque;
use advent_of_code::intcode::Computer;

fn run_computer(computer: &Computer, phase: i64, signal: i64, _trace: bool) -> i64 {
    let mut computer = computer.clone();
    let mut input = VecDeque::new();
    input.push_back(phase);
    input.push_back(signal);
    computer.run(&mut input).expect("one output")
}

fn max_thruster_signal(program_text: &str, trace: bool) -> i64 {
    let template_computer = Computer::parse(program_text);

    let permutations = vec![0, 1, 2, 3, 4].into_iter().permutations(5);

    let mut best_input_signal = i64::MIN;

    for phase_setting in permutations {
        let mut input_signal = 0;
        for phase in phase_setting.clone() {
            input_signal = run_computer(&template_computer, phase, input_signal, trace);
        }
        if input_signal >= best_input_signal {
            best_input_signal = input_signal;
            if trace {
                println!(
                    "new best input signal: {} from {:?}",
                    input_signal, phase_setting
                );
            }
        }
    }
    best_input_signal
}

fn part_one(input: &str) -> u32 {
    let signal = max_thruster_signal(input, false);
    signal.try_into().unwrap()
}

// fn part_two(_input: &str) -> u32 {
//     0
// }

fn main() {
    let input = include_str!("../inputs/07.txt").trim();
    let one = part_one(input);
    assert_eq!(one, 21760);
    // let two = part_two(input);
    // assert_eq!(two, 666);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        assert_eq!(
            max_thruster_signal("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", false),
            43210
        );
    }

    #[test]
    fn test_part_one_b() {
        assert_eq!(
            max_thruster_signal(
                "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
                false
            ),
            54321
        );
    }

    #[test]
    fn test_part_one_c() {
        assert_eq!(
            max_thruster_signal(
                "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",
                false
            ),
            65210
        );
    }

    #[test]
    fn test_main() {
        main();
    }
}
