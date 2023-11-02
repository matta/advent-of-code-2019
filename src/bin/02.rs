#![allow(dead_code)]

#[derive(PartialEq, Eq)]
enum Opcode {
    Add,
    Multiply,
    Finished,
}

#[derive(Clone)]
struct Computer {
    pc: u32,
    data: Vec<u32>,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            pc: 0,
            data: Vec::new(),
        }
    }

    fn get(&self, address: u32) -> u32 {
        self.data[address as usize]
    }

    fn opcode(&self) -> Opcode {
        let instruction = self.get(self.pc);

        match instruction {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            99 => Opcode::Finished,
            _ => panic!("Invalid instruction: {} at pc {}", instruction, self.pc),
        }
    }

    fn opderef(&self, index: u32) -> u32 {
        self.get(self.get(self.pc + index))
    }

    fn opput(&mut self, value: u32) {
        let address = self.get(self.pc + 3) as usize;
        self.data[address] = value
    }

    fn step(&mut self) {
        match self.opcode() {
            Opcode::Add => {
                self.opput(self.opderef(1) + self.opderef(2));
            }
            Opcode::Multiply => {
                self.opput(self.opderef(1) * self.opderef(2));
            }
            Opcode::Finished => {
                panic!("Step called on a finished program!");
            }
        }
        self.pc += 4
    }

    fn done(&self) -> bool {
        self.opcode() == Opcode::Finished
    }

    fn run(&mut self, noun: u32, verb: u32) {
        self.data[1] = noun;
        self.data[2] = verb;
        while !self.done() {
            self.step();
        }
    }
}

fn parse_computer_string(string: &str) -> Computer {
    let mut computer = Computer::new();
    for number in string.split(',') {
        computer.data.push(number.parse().unwrap());
    }
    computer
}

fn part_one(input: &str) -> u32 {
    let mut computer = parse_computer_string(input);
    computer.run(12, 2);
    computer.data[0]
}

fn compute(computer: &Computer, noun: u32, verb: u32) -> u32 {
    let mut computer = computer.clone();
    computer.run(noun, verb);
    computer.data[0]
}

fn part_two(input: &str) -> u32 {
    let computer = parse_computer_string(input);
    for noun in 0..=99 {
        for verb in 0..=99 {
            if compute(&computer, noun, verb) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("bug");
}

fn main() {
    let input = include_str!("../inputs/02.txt").trim();
    let one = part_one(input);
    let two = part_two(input);
    assert_eq!(one, 3895705);
    assert_eq!(two, 6417);
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

        assert!(computer.done());
    }

    #[test]
    fn test_main() {
        main();
    }

}
