#![allow(dead_code)]


#[derive(PartialEq, Eq)]
enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    Finished,
}

#[derive(Clone)]
struct Computer {
    pc: i32,
    data: Vec<i32>,
    output: Vec<i32>,
    finished: bool,
}

struct Param {
    immediate: bool,
    value: i32,
}

fn get_slice_from_index<T>(data: &[T], index: usize) -> &[T] {
    let mut slice = &data[index..];

    if slice.len() > 4 {
        slice = &slice[0..4];
    }

    slice
}
impl Computer {
    fn new() -> Computer {
        Computer {
            pc: 0,
            data: Vec::new(),
            output: Vec::new(),
            finished: false,
        }
    }

    fn get(&self, address: i32) -> i32 {
        // println!("    data[{}] -> {}", address, self.data[address as usize]);
        self.data[address as usize]
    }

    fn opcode(&self) -> Opcode {
        let instruction = self.get(self.pc) % 100;

        match instruction {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            3 => Opcode::Input,
            4 => Opcode::Output,
            99 => Opcode::Finished,
            _ => panic!("Invalid instruction: {} at pc {}", instruction, self.pc),
        }
    }

    fn store(&mut self, param: i32, value: i32) {
        // println!("  store: param={} value={}", param, value);
        let address = self.get(self.pc + param) as usize;
        // println!("    {} -> data[{}]", value, address);
        self.data[address] = value
    }

    fn load(&self, param: i32) -> i32 {
        // println!("  load: param={}", param);
        let instruction = self.get(self.pc);
        // println!("    instruction: {}", instruction);
        assert!(instruction >= 0);

        let mut tmp = instruction / 100;
        for _ in 1..param {
            tmp /= 10;
        }
        let immediate = tmp % 10 == 1;

        // println!("    immediate: {}", immediate);
        let value = self.get(self.pc + param);
        if immediate {
            value
        } else {
            self.get(value)
        }
    }

    fn step(&mut self) {
        // println!("step: pc={}", self.pc);
        // let mut debug_data = &self.data[(self.pc as usize)..];
        // if debug_data.len() > 4 {
        //     debug_data = &debug_data[0..4];
        // }
        // println!("  program at pc: {:?}", debug_data);

        match self.opcode() {
            Opcode::Add => {
                // println!("  add");
                self.store(3, self.load(1) + self.load(2));
                self.pc += 4
            }
            Opcode::Multiply => {
                // println!("  multiply");
                self.store(3, self.load(1) * self.load(2));
                self.pc += 4
            }
            Opcode::Input => {
                // println!("  input");
                self.store(1, 1);
                self.pc += 2
            }
            Opcode::Output => {
                // println!("  output");
                let value = self.load(1);
                // println!("    value: {}", value);
                self.output.push(value);
                self.pc += 2;
                if self.data[self.pc as usize] != 99 {
                    assert_eq!(value, 0);
                }
            }
            Opcode::Finished => {
                // println!("  finished");
                self.finished = true;
            }
        }
    }

    fn run(&mut self) {
        // let debug_data = &self.data[..].get(..16).unwrap_or(&self.data);
        // println!("run: data={:?}...", debug_data);

        while !self.finished {
            self.step();
        }
    }
}

fn parse_computer_string(string: &str) -> Computer {
    let mut computer = Computer::new();
    for number in string.trim_end().split(',') {
        computer.data.push(number.parse().unwrap_or_else(|e| {
            panic!("Invalid numerical string: \"{}\" error: {}", number, e);
        }));
    }
    computer
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut computer = parse_computer_string(input);
    computer.run();
    // println!("output: {:?}", computer.output);
    computer.output.last().map(|x| *x as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        let mut computer = parse_computer_string("1,5,6,7,99,11,13,0");
        computer.step();
        assert_eq!(computer.pc, 4);
        assert_eq!(computer.data, vec![1, 5, 6, 7, 99, 11, 13, 11 + 13]);
    }

    #[test]
    fn test_step_multiply() {
        let mut computer = parse_computer_string("2,5,6,7,99,11,13,10000");
        computer.step();
        assert_eq!(computer.pc, 4);
        assert_eq!(computer.data, vec![2, 5, 6, 7, 99, 11, 13, 11 * 13]);
    }

    #[test]
    fn test_part_one() {
        let mut computer = parse_computer_string("1,9,10,3,2,3,11,0,99,30,40,50");
        computer.step();
        assert_eq!(computer.pc, 4);
        assert_eq!(
            computer.data,
            vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );

        computer.step();
        assert_eq!(computer.pc, 8);
        assert_eq!(
            computer.data,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );

        assert!(computer.finished);
    }

    #[test]
    fn test_output_example() {
        let mut computer = parse_computer_string("3,0,4,0,99");
        computer.step();
        assert_eq!(computer.pc, 2);
        assert_eq!(computer.data, vec![1, 0, 4, 0, 99]);
        assert_eq!(computer.output, vec![]);

        computer.step();
        assert_eq!(computer.pc, 4);
        assert_eq!(computer.data, vec![1, 0, 4, 0, 99]);
        assert_eq!(computer.output, vec![1]);

        assert!(computer.finished);
        assert_eq!(computer.pc, 4);
        assert_eq!(computer.data, vec![1, 0, 4, 0, 99]);
        assert_eq!(computer.output, vec![1]);
    }
}
