use std::collections::HashMap;

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

#[derive(Debug)]
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

#[derive(Debug)]
struct Computer {
    pc: i64,
    memory: Memory,
    relative_base: i64,
    input: i64,
    output: Vec<i64>,
    finished: bool,
    trace: bool,
    step: i32,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            pc: 0,
            memory: Memory::new(false),
            relative_base: 0,
            input: 0,
            output: Vec::new(),
            finished: false,
            trace: false,
            step: 0,
        }
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

    fn step(&mut self) -> anyhow::Result<()> {
        self.step += 1;
        if self.step > 1_000_000 {
            bail!("Too many steps");
        }
        if self.trace {
            println!("step {}: pc={}", self.step, self.pc);
        }

        let instruction = parse_instruction(self.pc, &self.memory)?;
        if self.trace {
            println!("  instruction: {:?}", instruction);
        }

        self.pc += match instruction {
            Instruction::Add(a, b, c) => {
                let value = self.load(a) + self.load(b);
                self.store(c, value);
                4
            }
            Instruction::Multiply(a, b, c) => {
                let value = self.load(a) * self.load(b);
                self.store(c, value);
                4
            }
            Instruction::Input(a) => {
                self.store(a, self.input);
                2
            }
            Instruction::Output(a) => {
                let value = self.load(a);
                if self.trace {
                    println!(" output: {}", value);
                }
                self.output.push(value);
                2
            }
            Instruction::JumpIfTrue(a, b) => {
                let value = self.load(a);
                if value != 0 {
                    self.pc = self.load(b);
                    0
                } else {
                    3
                }
            }
            Instruction::JumpIfFalse(a, b) => {
                let value = self.load(a);
                if value == 0 {
                    self.pc = self.load(b);
                    0
                } else {
                    3
                }
            }
            Instruction::LessThan(a, b, c) => {
                let less = self.load(a) < self.load(b);
                self.store(c, if less { 1 } else { 0 });
                4
            }
            Instruction::Equals(a, b, c) => {
                let equals = self.load(a) == self.load(b);
                self.store(c, if equals { 1 } else { 0 });
                4
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
                2
            }
            Instruction::Finished => {
                if self.trace {
                    println!("FINISHED");
                }
                self.finished = true;
                0
            }
        };
        Ok(())
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

fn run_program(program_text: &str, input_number: i64, trace: bool) -> Result<Vec<i64>> {
    let mut computer = parse_program(program_text);
    computer.input = input_number;
    computer.trace = trace;
    computer.memory.trace = trace;
    if trace {
        println!("run_program: {:?}", computer);
    }
    while !computer.finished {
        computer.step()?;
    }
    Ok(computer.output)
}

pub fn part_one(_input: &str) -> Option<i32> {
    let output = run_program(_input, 0, false).unwrap();

    let mut tiles: HashMap<(i64, i64), i64> = HashMap::new();

    assert_eq!(output.len() % 3, 0);
    for chunk in output.chunks_exact(3) {
        match chunk {
            [x, y, tile_id] => {
                *tiles.entry((*x, *y)).or_insert(*tile_id) = *tile_id;
            }
            _ => {
                panic!("Invalid output {:?}", chunk);
            }
        }
    }

    let mut block_tiles = 0;
    for v in tiles.values() {
        if *v == 2 {
            block_tiles += 1;
        }
    }
    Some(block_tiles)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    // use super::*;
}
