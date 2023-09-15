// #![allow(dead_code)]

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
    input: i32,
    output: Vec<i32>,
    finished: bool,
    trace: bool,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            pc: 0,
            memory: Vec::new(),
            input: 0,
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
                self.store(1, self.input);
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
    computer.input = 1; // the default for part one
    computer
}

pub fn run_program(program_text: &str, input_number: i32, trace: bool) -> Vec<i32> {
    let mut computer = parse_program(program_text);
    computer.input = input_number;
    computer.trace = trace;
    if trace {
        println!("run_program: {:?}", computer);
    }
    computer.run();
    computer.output
}

pub fn part_one(input: &str) -> Option<u32> {
    let output = run_program(input, 1, false);
    output.last().map(|x| *x as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let output = run_program(input, 5, false);
    output.last().map(|x| *x as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_add() {
        let mut computer = parse_program("1,5,6,7,99,11,13,0");
        computer.step();
        assert_eq!(computer.pc, 4);
        assert_eq!(computer.memory, vec![1, 5, 6, 7, 99, 11, 13, 11 + 13]);
    }

    #[test]
    fn test_step_multiply() {
        let mut computer = parse_program("2,5,6,7,99,11,13,10000");
        computer.step();
        assert_eq!(computer.pc, 4);
        assert_eq!(computer.memory, vec![2, 5, 6, 7, 99, 11, 13, 11 * 13]);
    }

    #[test]
    fn test_part_one() {
        let mut computer = parse_program("1,9,10,3,2,3,11,0,99,30,40,50");
        computer.step();
        assert_eq!(computer.pc, 4);
        assert_eq!(
            computer.memory,
            vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );

        computer.step();
        assert_eq!(computer.pc, 8);
        assert_eq!(
            computer.memory,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );

        computer.step();
        assert!(computer.finished);
    }

    #[test]
    fn test_output_example() {
        let mut computer = parse_program("3,0,4,0,99");
        computer.step();
        assert_eq!(computer.pc, 2);
        assert_eq!(computer.memory, vec![1, 0, 4, 0, 99]);
        assert_eq!(computer.output, vec![]);

        computer.step();
        assert_eq!(computer.pc, 4);
        assert_eq!(computer.memory, vec![1, 0, 4, 0, 99]);
        assert_eq!(computer.output, vec![1]);

        computer.step();
        assert!(computer.finished);
        assert_eq!(computer.pc, 4);
        assert_eq!(computer.memory, vec![1, 0, 4, 0, 99]);
        assert_eq!(computer.output, vec![1]);
    }

    #[test]
    fn test_equal_position_mode() {
        let trace = false;
        let program_text = "3,9,8,9,10,9,4,9,99,-1,8";
        let magic_number = 8;
        assert_eq!(run_program(program_text, magic_number, trace), vec![1]);
        assert_eq!(run_program(program_text, magic_number + 1, trace), vec![0]);
    }

    #[test]
    fn test_equal_immediate_mode() {
        let trace = false;
        let program_text = "3,3,1108,-1,8,3,4,3,99";
        let magic_number = 8;
        assert_eq!(run_program(program_text, magic_number, trace), vec![1]);
        assert_eq!(run_program(program_text, magic_number + 1, trace), vec![0]);
    }

    #[test]
    fn test_less_than_position_mode() {
        let trace = false;
        let program_text = "3,9,7,9,10,9,4,9,99,-1,8";
        let magic_number = 8;
        let output = run_program(program_text, magic_number - 1, trace);
        assert_eq!(output, vec![1]);
        let output = run_program(program_text, magic_number, trace);
        assert_eq!(output, vec![0]);
    }

    #[test]
    fn test_less_than_immediate_mode() {
        let trace = false;
        let program_text = "3,3,1107,-1,8,3,4,3,99";
        let magic_number = 8;
        let output = run_program(program_text, magic_number - 1, trace);
        assert_eq!(output, vec![1]);
        let output = run_program(program_text, magic_number, trace);
        assert_eq!(output, vec![0]);
    }

    #[test]
    fn test_jump_position_mode() {
        let trace = false;
        let program_text = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        let magic_number = 0;
        let output = run_program(program_text, magic_number, trace);
        assert_eq!(output, vec![0]);
        let output = run_program(program_text, magic_number + 1, trace);
        assert_eq!(output, vec![1]);
    }

    #[test]
    fn test_jump_immediate_mode() {
        let trace = false;
        let program_text = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        let magic_number = 0;
        let output = run_program(program_text, magic_number, trace);
        assert_eq!(output, vec![0]);
        let output = run_program(program_text, magic_number + 1, trace);
        assert_eq!(output, vec![1]);
    }

    #[test]
    fn test_larger_example() {
        let trace = false;
        let program_text = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let magic_number = 8;
        let output = run_program(program_text, magic_number - 1, trace);
        assert_eq!(output, vec![999]);
        let output = run_program(program_text, magic_number, trace);
        assert_eq!(output, vec![1000]);
        let output = run_program(program_text, magic_number + 1, trace);
        assert_eq!(output, vec![1001]);
    }
}
