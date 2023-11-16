use aoc2019::intcode::{Computer, RunState};
use itertools::Itertools;

fn run_computer(computer: &Computer, phase: i64, signal: i64, _trace: bool) -> i64 {
    let mut computer = computer.clone();
    computer.append_input(&[phase, signal]);
    match computer.run() {
        RunState::Finished => panic!("computer error: finished prematurely"),
        RunState::BlockedOnInput => panic!("computer error: blocked on input unexpectedly"),
        RunState::BlockedOnOutput => computer.take_output().unwrap(),
    }
}

fn max_thruster_signal(program_text: &str, trace: bool) -> i64 {
    let template_computer = Computer::parse(program_text);

    let permutations = vec![0, 1, 2, 3, 4].into_iter().permutations(5);

    let mut best_input_signal = i64::MIN;

    for phase_settings in permutations {
        let mut input_signal = 0;
        for phase in &phase_settings {
            input_signal = run_computer(&template_computer, *phase, input_signal, trace);
        }
        if input_signal >= best_input_signal {
            best_input_signal = input_signal;
            if trace {
                println!(
                    "new best input signal: {} from {:?}",
                    input_signal, phase_settings
                );
            }
        }
    }
    best_input_signal
}

fn max_thruster_signal2(program_text: &str) -> i64 {
    let template_computer = Computer::parse(program_text);

    let permutations = vec![5, 6, 7, 8, 9].into_iter().permutations(5);

    let mut highest_output_signal = i64::MIN;

    for phase_settings in permutations {
        let mut computers: Vec<Computer> = phase_settings
            .iter()
            .map(|phase| {
                let mut computer = template_computer.clone();
                computer.append_input(&[*phase]);
                computer
            })
            .collect();

        let mut input_signal = 0;
        'until_finished: loop {
            for computer in computers.iter_mut() {
                computer.append_input(&[input_signal]);
                match computer.run() {
                    RunState::BlockedOnOutput => {
                        let output_signal = computer.take_output().unwrap();
                        if output_signal > highest_output_signal {
                            highest_output_signal = output_signal;
                        }
                        input_signal = output_signal;
                    }
                    RunState::BlockedOnInput => {
                        panic!("computer malfunction: unexpected block on input");
                    }
                    RunState::Finished => {
                        break 'until_finished;
                    }
                }
            }
        }
    }
    highest_output_signal
}

fn part_one(input: &str) -> u32 {
    let signal = max_thruster_signal(input, false);
    signal.try_into().unwrap()
}

fn part_two(input: &str) -> u32 {
    let signal = max_thruster_signal2(input);
    signal.try_into().unwrap()
}

fn main() {
    let input = include_str!("../inputs/07.txt").trim();
    let one = part_one(input);
    assert_eq!(one, 21760);
    let two = part_two(input);
    assert_eq!(two, 69816958);
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
    fn test_part_two_a() {
        assert_eq!(
            max_thruster_signal2(
                "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
            ),
            139629729
        )
    }

    #[test]
    fn test_part_two_b() {
        assert_eq!(
            max_thruster_signal2(
                "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
            ),
            18216
        )
    }

    #[test]
    fn test_main() {
        main();
    }
}
