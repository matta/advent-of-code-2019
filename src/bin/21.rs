use aoc2019::intcode::{Computer, RunState};

const INTCODE_PROGRAM: &str = include_str!("../inputs/21.txt");

const PART_ONE_SPRINGSCRIPT: &str = "\
NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK
";

const PART_TWO_SPRINGSCRIPT: &str = "\
NOT H T
OR C T
AND B T
AND A T
NOT T J
AND D J
RUN
";

fn run_springscript(program: &str) -> i64 {
    let mut computer = Computer::parse(INTCODE_PROGRAM);

    for b in program.as_bytes().iter().map(|b| *b as i64) {
        computer.append_input(&[b]);
    }

    // let mut input = Vec::new();
    // let stdin = std::io::stdin();
    // let mut handle = stdin.lock();
    // handle.read_to_end(&mut input).expect("oopse, no input?");
    // computer.append_input(&input.into_iter().map(|e| e as i64).collect::<Vec<i64>>());

    loop {
        match computer.run() {
            RunState::BlockedOnInput => {
                panic!("premature end of input");
            }
            RunState::BlockedOnOutput => {
                if let Some(str) = computer.read_ascii_string() {
                    print!("{}", str);
                } else {
                    return computer.take_output().unwrap();
                }
            }
            RunState::Finished => {
                panic!("premature program termination");
            }
        }
    }
}

fn main() {
    assert_eq!(run_springscript(PART_ONE_SPRINGSCRIPT), 19352638);
    assert_eq!(run_springscript(PART_TWO_SPRINGSCRIPT), 1141251258);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
