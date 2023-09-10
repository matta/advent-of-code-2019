#![allow(dead_code)]

#[derive(PartialEq, Eq)]
enum Opcode {
    Add,
    Multiply,
    Finished,
}

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
        // println!("data[{}] -> {}", address, self.data[address as usize]);
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
        // println!("data[{}] <- {}", address, value);
        self.data[address] = value
    }

    fn step(&mut self) {
        // println!("step: pc={}", self.pc);
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
}

fn parse_computer_string(string: &str) -> Computer {
    let mut computer = Computer::new();
    for number in string.split(',') {
        computer.data.push(number.parse().unwrap());
    }
    computer
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut computer = parse_computer_string(input);
    computer.data[1] = 12;
    computer.data[2] = 2;
    while !computer.done() {
        computer.step();
    }
    Some(computer.data[0])
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
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

        assert!(computer.done());
    }

    // #[test]
    //     fn test_part_two() {
    //         let input = advent_of_code::read_file("examples", 2);
    //         assert_eq!(part_two(&input), None);
    //     }
}
