// Example import from this file: `use aoc2019::intcode::Foo;`.

use std::collections::VecDeque;

use anyhow::bail;
use anyhow::Result;

#[derive(Clone, Debug, PartialEq, Eq)]
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
    AdjustRelativeBase,
}

impl Opcode {
    fn from(i: i64) -> anyhow::Result<Opcode> {
        match i {
            1 => anyhow::Ok(Opcode::Add),
            2 => anyhow::Ok(Opcode::Multiply),
            3 => anyhow::Ok(Opcode::Input),
            4 => anyhow::Ok(Opcode::Output),
            5 => anyhow::Ok(Opcode::JumpIfTrue),
            6 => anyhow::Ok(Opcode::JumpIfFalse),
            7 => anyhow::Ok(Opcode::LessThan),
            8 => anyhow::Ok(Opcode::Equals),
            9 => anyhow::Ok(Opcode::AdjustRelativeBase),
            99 => anyhow::Ok(Opcode::Finished),
            _ => anyhow::bail!("Invalid opcode: {}", i),
        }
    }
}

#[derive(Clone, Debug)]
enum ParameterMode {
    Position,
    Immediate,
    RelativePosition,
}

impl ParameterMode {
    fn from(i: i64) -> Result<ParameterMode> {
        let mode = match i {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::RelativePosition,
            _ => bail!("Invalid parameter mode: {}", i),
        };
        Ok(mode)
    }
}

#[derive(Clone, Debug)]
struct Parameter {
    mode: ParameterMode,
    value: i64,
}

#[derive(Clone, Debug)]
enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    Finished,
    AdjustRelativeBase(Parameter),
}

#[derive(Debug, Clone)]
struct Memory {
    vec: Vec<i64>,
    trace: bool,
}

impl Memory {
    fn new(trace: bool) -> Self {
        Memory {
            vec: Vec::new(),
            trace,
        }
    }

    fn get(&self, index: i64) -> i64 {
        if index < 0 {
            panic!("invalid index: {}", index);
        }
        let value = if index >= self.vec.len() as i64 {
            0
        } else {
            self.vec[index as usize]
        };
        if self.trace {
            println!("        mem[{}] -> {}", index, value);
        }
        value
    }

    fn set(&mut self, index: i64, value: i64) {
        if !(0..(100 * 1000)).contains(&index) {
            panic!("invalid index: {}", index);
        }
        while index >= self.vec.len() as i64 {
            self.vec.push(0);
        }
        self.vec[index as usize] = value;
        if self.trace {
            println!("        mem[{}] <- {}", index, value);
        }
    }

    fn push(&mut self, value: i64) {
        self.vec.push(value);
    }
}

fn parse_parameter(
    param: i64,
    instruction: i64,
    pc: i64,
    memory: &Memory,
) -> anyhow::Result<Parameter> {
    if !(0..=2).contains(&param) {
        panic!("Invalid parameter index: {}", param);
    }

    let mut tmp = instruction / 100;
    for _ in 0..param {
        tmp /= 10;
    }
    let immediate_value = memory.get(pc + 1 + param);
    let mode = ParameterMode::from(tmp % 10)?;
    let parameter = Parameter {
        mode,
        value: immediate_value,
    };
    Ok(parameter)
}

fn parse_instruction(pc: i64, memory: &Memory) -> anyhow::Result<Instruction> {
    let instruction = memory.get(pc);
    let op = Opcode::from(instruction % 100)?;
    let instr = match op {
        Opcode::Add => {
            let a = parse_parameter(0, instruction, pc, memory)?;
            let b = parse_parameter(1, instruction, pc, memory)?;
            let c = parse_parameter(2, instruction, pc, memory)?;
            Instruction::Add(a, b, c)
        }
        Opcode::Multiply => {
            let a = parse_parameter(0, instruction, pc, memory)?;
            let b = parse_parameter(1, instruction, pc, memory)?;
            let c = parse_parameter(2, instruction, pc, memory)?;
            Instruction::Multiply(a, b, c)
        }
        Opcode::Input => {
            let a = parse_parameter(0, instruction, pc, memory)?;
            Instruction::Input(a)
        }
        Opcode::Output => {
            let a = parse_parameter(0, instruction, pc, memory)?;
            Instruction::Output(a)
        }
        Opcode::JumpIfTrue => {
            let a = parse_parameter(0, instruction, pc, memory)?;
            let b = parse_parameter(1, instruction, pc, memory)?;
            Instruction::JumpIfTrue(a, b)
        }
        Opcode::JumpIfFalse => {
            let a = parse_parameter(0, instruction, pc, memory)?;
            let b = parse_parameter(1, instruction, pc, memory)?;
            Instruction::JumpIfFalse(a, b)
        }
        Opcode::LessThan => {
            let a = parse_parameter(0, instruction, pc, memory)?;
            let b = parse_parameter(1, instruction, pc, memory)?;
            let c = parse_parameter(2, instruction, pc, memory)?;
            Instruction::LessThan(a, b, c)
        }
        Opcode::Equals => {
            let a = parse_parameter(0, instruction, pc, memory)?;
            let b = parse_parameter(1, instruction, pc, memory)?;
            let c = parse_parameter(2, instruction, pc, memory)?;
            Instruction::Equals(a, b, c)
        }
        Opcode::Finished => Instruction::Finished,
        Opcode::AdjustRelativeBase => {
            let a = parse_parameter(0, instruction, pc, memory)?;
            Instruction::AdjustRelativeBase(a)
        }
    };
    Ok(instr)
}

// Input and output for a computer.
//
// Originally I had input and output as separate traits, but ran into issues
// with mutability and borrow checking when handling both traits on one
// object.
pub trait ComputerIO {
    fn input(&mut self) -> i64;
    fn output(&mut self, value: i64);
}

type Word = i64;

#[derive(Clone)]
pub struct Computer {
    pc: i64,
    memory: Memory,
    relative_base: i64,
    input_buffer: VecDeque<i64>,
    output: Option<Word>,
    finished: bool,
    trace: bool,
    step: i32,
}

#[derive(Debug, PartialEq)]
pub enum RunState {
    BlockedOnInput,
    BlockedOnOutput,
    Finished,
}

#[derive(Debug, PartialEq)]
pub enum StepState {
    Running,
    BlockedOnInput,
    BlockedOnOutput,
    Finished,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            pc: 0,
            memory: Memory::new(false),
            relative_base: 0,
            input_buffer: VecDeque::new(),
            output: None,
            finished: false,
            trace: false,
            step: 0,
        }
    }

    pub fn parse(text: &str) -> Computer {
        let mut computer = Computer::new();
        for number in text.trim_end().split(',') {
            computer.memory.push(number.parse().unwrap_or_else(|e| {
                panic!("Invalid numerical string: \"{}\" error: {}", number, e);
            }));
        }
        computer
    }

    fn store(&mut self, param: Parameter, value: i64) {
        let address = match param.mode {
            ParameterMode::Position => param.value,
            ParameterMode::Immediate => {
                panic!("storing to an immediate parameter is not not implemented")
            }
            ParameterMode::RelativePosition => self.relative_base + param.value,
        };
        self.store_to_address(address, value);
    }

    fn store_to_address(&mut self, address: i64, value: i64) {
        if self.trace {
            println!("    store: mem[{}] <- {}", address, value);
        }
        if !(0..=(128 * 1024)).contains(&address) {
            panic!("Invalid address: {}", address);
        }
        self.memory.set(address, value);
    }

    fn load(&self, param: Parameter) -> i64 {
        if self.trace {
            println!("    load: param={:?}", param);
        }
        match param.mode {
            ParameterMode::Position => self.memory.get(param.value),
            ParameterMode::Immediate => param.value,
            ParameterMode::RelativePosition => self.memory.get(self.relative_base + param.value),
        }
    }

    pub fn poke(&mut self, index: i64, value: i64) {
        self.memory.set(index, value);
    }

    pub fn run(&mut self) -> RunState {
        loop {
            match self.step() {
                StepState::Running => {}
                StepState::BlockedOnInput => return RunState::BlockedOnInput,
                StepState::BlockedOnOutput => return RunState::BlockedOnOutput,
                StepState::Finished => return RunState::Finished,
            }
        }
    }

    pub fn append_str(&mut self, str: &str) {
        for num in str.chars().map(|ch| ch as i64) {
            self.append_input(&[num]);
        }
    }

    pub fn append_input(&mut self, numbers: &[i64]) {
        self.input_buffer.extend(numbers.iter());
    }

    pub fn take_output(&mut self) -> Option<Word> {
        self.output.take()
    }

    /// Steps this [`Computer`] until it is finished, blocked on input, or
    /// produces non-ascii output. Returns the output as a String containing
    /// ASCII characters.
    pub fn read_ascii_string(&mut self) -> Option<String> {
        let mut out = String::new();
        loop {
            match self.step() {
                StepState::Running => {}
                StepState::BlockedOnInput | StepState::Finished => break,
                StepState::BlockedOnOutput => {
                    let output = self.output.unwrap();
                    if (0..128).contains(&output) {
                        out.push(self.take_output().unwrap() as u8 as char);
                    } else {
                        break;
                    }
                }
            }
        }
        if !out.is_empty() {
            Some(out)
        } else {
            None
        }
    }

    pub fn step(&mut self) -> StepState {
        if self.output.is_some() {
            return StepState::BlockedOnOutput;
        }

        self.step += 1;
        if self.step > 100_000_000 {
            panic!("Too many steps");
        }
        if self.trace {
            println!("step {}: pc={}", self.step, self.pc);
        }

        let instruction =
            parse_instruction(self.pc, &self.memory).expect("parse instruction failed");
        if self.trace {
            println!("  instruction: {:?}", instruction);
        }

        match instruction {
            Instruction::Add(a, b, c) => {
                let value = self.load(a) + self.load(b);
                self.store(c, value);
                self.pc += 4;
            }
            Instruction::Multiply(a, b, c) => {
                let value = self.load(a) * self.load(b);
                self.store(c, value);
                self.pc += 4;
            }
            Instruction::Input(a) => {
                if let Some(value) = self.input_buffer.pop_front() {
                    self.store(a, value);
                    self.pc += 2
                } else {
                    return StepState::BlockedOnInput;
                }
            }
            Instruction::Output(a) => {
                let value = self.load(a);
                if self.trace {
                    println!(" output: {}", value);
                }
                self.output = Some(value);
                self.pc += 2;
                return StepState::BlockedOnOutput;
            }
            Instruction::JumpIfTrue(a, b) => {
                let value = self.load(a);
                if value != 0 {
                    self.pc = self.load(b);
                } else {
                    self.pc += 3;
                }
            }
            Instruction::JumpIfFalse(a, b) => {
                let value = self.load(a);
                if value == 0 {
                    self.pc = self.load(b);
                } else {
                    self.pc += 3;
                }
            }
            Instruction::LessThan(a, b, c) => {
                let less = self.load(a) < self.load(b);
                self.store(c, if less { 1 } else { 0 });
                self.pc += 4;
            }
            Instruction::Equals(a, b, c) => {
                let equals = self.load(a) == self.load(b);
                self.store(c, if equals { 1 } else { 0 });
                self.pc += 4;
            }
            Instruction::AdjustRelativeBase(a) => {
                let value = self.load(a);
                if self.trace {
                    println!(
                        "    relative-base <= {} + {} <= {}",
                        self.relative_base,
                        value,
                        self.relative_base + value
                    );
                }
                self.relative_base += value;
                self.pc += 2;
            }
            Instruction::Finished => {
                if self.trace {
                    println!("FINISHED");
                }
                self.finished = true;
                return StepState::Finished;
            }
        }
        StepState::Running
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_add() {
        let mut computer = Computer::parse("1,5,6,7,99,11,13,0");
        computer.trace = true;
        computer.memory.trace = true;
        assert_eq!(computer.step(), StepState::Running);
        assert_eq!(computer.pc, 4);
        assert_eq!(computer.memory.vec, vec![1, 5, 6, 7, 99, 11, 13, 11 + 13]);
    }

    #[test]
    fn test_step_multiply() {
        let mut computer = Computer::parse("2,5,6,7,99,11,13,0");
        computer.trace = true;
        computer.memory.trace = true;
        assert_eq!(computer.step(), StepState::Running);
        assert_eq!(computer.pc, 4);
        assert_eq!(computer.memory.vec, vec![2, 5, 6, 7, 99, 11, 13, 11 * 13]);
    }

    fn run_program(program_text: &str, input_number: i64, trace: bool) -> Vec<i64> {
        let mut computer = Computer::parse(program_text);
        computer.trace = trace;
        computer.memory.trace = trace;
        computer.append_input(&[input_number]);
        let mut output = Vec::new();
        loop {
            match computer.run() {
                RunState::BlockedOnInput => panic!("Input exhausted!"),
                RunState::BlockedOnOutput => output.push(computer.take_output().unwrap()),
                RunState::Finished => break,
            }
        }
        output
    }

    // #[test]
    // fn test_part_one_quine() {
    //     assert_eq!(
    //         run_program(
    //             "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
    //             0,
    //             true
    //         ),
    //         vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
    //     );
    // }

    #[test]
    fn test_part_one_16_digit() {
        assert_eq!(
            run_program("1102,34915192,34915192,7,4,7,99,0", 0, true),
            vec![1219070632396864]
        );
    }

    #[test]
    fn test_part_one_digit_in_the_middle() {
        assert_eq!(
            run_program("104,1125899906842624,99", 0, true),
            vec![1125899906842624]
        );
    }

    #[test]
    fn test_output_input() {
        assert_eq!(run_program("3,0,4,0,99", 42, true), vec![42]);
    }
}
