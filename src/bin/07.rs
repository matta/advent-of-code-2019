use itertools::Itertools;
use std::collections::VecDeque;

#[derive(PartialEq, Eq)]
enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Finished,
}

#[derive(Clone, Debug)]
struct Computer {
    pc: i32,
    memory: Vec<i32>,
    input: VecDeque<i32>,
    output: Vec<i32>,
    finished: bool,
    trace: bool,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            pc: 0,
            memory: Vec::new(),
            input: VecDeque::new(),
            output: Vec::new(),
            finished: false,
            trace: false,
        }
    }

    fn get(&self, address: i32) -> i32 {
        if self.trace {
            println!(
                "    memory[{}] -> {}",
                address, self.memory[address as usize]
            );
        }
        self.memory[address as usize]
    }

    fn opcode(&self) -> Opcode {
        let instruction = self.get(self.pc) % 100;

        match instruction {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::JumpIfTrue,
            6 => Opcode::JumpIfFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            99 => Opcode::Finished,
            _ => panic!("Invalid instruction: {} at pc {}", instruction, self.pc),
        }
    }

    fn store(&mut self, param: i32, value: i32) {
        if self.trace {
            println!("  store: param={} value={}", param, value);
        }
        let address = self.get(self.pc + param) as usize;
        if self.trace {
            println!("    {} -> memory[{}]", value, address);
        }
        self.memory[address] = value
    }

    fn load(&self, param: i32) -> i32 {
        if self.trace {
            println!("  load: param={}", param);
        }
        let instruction = self.get(self.pc);
        if self.trace {
            println!("    instruction: {}", instruction);
        }
        assert!(instruction >= 0);

        let mut tmp = instruction / 100;
        for _ in 1..param {
            tmp /= 10;
        }
        let immediate = tmp % 10 == 1;

        if self.trace {
            println!("    immediate: {}", immediate);
        }
        let value = self.get(self.pc + param);
        if immediate {
            value
        } else {
            self.get(value)
        }
    }

    fn step(&mut self) {
        if self.trace {
            println!("step: pc={}", self.pc);
            let mut debug_memory = &self.memory[(self.pc as usize)..];
            if debug_memory.len() > 4 {
                debug_memory = &debug_memory[0..4];
            }
            println!("  program at pc: {:?}", debug_memory);
        }

        match self.opcode() {
            Opcode::Add => {
                if self.trace {
                    println!("  add");
                }
                self.store(3, self.load(1) + self.load(2));
                self.pc += 4
            }
            Opcode::Multiply => {
                if self.trace {
                    println!("  multiply");
                }
                self.store(3, self.load(1) * self.load(2));
                self.pc += 4
            }
            Opcode::Input => {
                if self.trace {
                    println!("  input");
                }
                let value = self.input.pop_front().unwrap();
                self.store(1, value);
                self.pc += 2
            }
            Opcode::Output => {
                if self.trace {
                    println!("  output");
                }
                let value = self.load(1);
                if self.trace {
                    println!("    value: {}", value);
                }
                self.output.push(value);
                self.pc += 2;
            }
            Opcode::JumpIfTrue => {
                if self.trace {
                    println!("  jump-if-true");
                }
                let value = self.load(1);
                self.pc = if value != 0 {
                    self.load(2)
                } else {
                    self.pc + 3
                }
            }
            Opcode::JumpIfFalse => {
                if self.trace {
                    println!("  jump-if-false");
                }
                let value = self.load(1);
                self.pc = if value == 0 {
                    self.load(2)
                } else {
                    self.pc + 3
                }
            }
            Opcode::LessThan => {
                if self.trace {
                    println!("  less-than");
                }
                let less = self.load(1) < self.load(2);
                if self.trace {
                    println!("    less: {}", less);
                }
                self.store(3, if less { 1 } else { 0 });
                self.pc += 4
            }
            Opcode::Equals => {
                if self.trace {
                    println!("  equals");
                }
                let equals = self.load(1) == self.load(2);
                if self.trace {
                    println!("    equals: {}", equals);
                }
                self.store(3, if equals { 1 } else { 0 });
                self.pc += 4
            }
            Opcode::Finished => {
                if self.trace {
                    println!("  finished");
                }
                self.finished = true;
            }
        }
    }

    fn run(&mut self) {
        while !self.finished {
            self.step();
        }
    }
}

fn parse_program(text: &str) -> Computer {
    let mut computer = Computer::new();
    for number in text.trim_end().split(',') {
        computer.memory.push(number.parse().unwrap_or_else(|e| {
            panic!("Invalid numerical string: \"{}\" error: {}", number, e);
        }));
    }
    computer
}

fn run_computer(computer: &Computer, phase: i32, signal: i32, trace: bool) -> i32 {
    let mut computer = computer.clone();
    computer.input.push_back(phase);
    computer.input.push_back(signal);
    computer.trace = trace;
    computer.run();
    computer.output[0]
}

fn max_thruster_signal(program_text: &str, trace: bool) -> i32 {
    let template_computer = parse_program(program_text);

    let permutations = vec![0, 1, 2, 3, 4].into_iter().permutations(5);

    let mut best_input_signal = i32::MIN;

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

fn part_one(input: &str) -> Option<u32> {
    let signal = max_thruster_signal(input, false);
    Some(signal as u32)
}

fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
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
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
