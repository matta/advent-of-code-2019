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

// Input and output for a computer.
//
// Originally I had input and output as separate traits, but ran into issues
// with mutability and borrow checking when handling both traits on one
// object.
trait ComputerIO {
    fn input(&mut self) -> i64;
    fn output(&mut self, value: i64);
}

struct Computer<'a> {
    pc: i64,
    memory: Memory,
    relative_base: i64,
    io: &'a mut dyn ComputerIO,
    finished: bool,
    trace: bool,
    step: i32,
}

impl Computer<'_> {
    fn new(io: &mut dyn ComputerIO) -> Computer<'_> {
        Computer {
            pc: 0,
            memory: Memory::new(false),
            relative_base: 0,
            io,
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
                let value = self.io.input();
                self.store(a, value);
                2
            }
            Instruction::Output(a) => {
                let value = self.load(a);
                if self.trace {
                    println!(" output: {}", value);
                }
                self.io.output(value);
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

struct VectorOutputer {
    vec: Vec<i64>,
}

impl ComputerIO for VectorOutputer {
    fn input(&mut self) -> i64 {
        panic!("VectorOutputer::input() not implemented");
    }

    fn output(&mut self, value: i64) {
        self.vec.push(value);
    }
}

fn parse_program<'a>(text: &'a str, io: &'a mut dyn ComputerIO) -> Computer<'a> {
    let mut computer = Computer::new(io);
    for number in text.trim_end().split(',') {
        computer.memory.push(number.parse().unwrap_or_else(|e| {
            panic!("Invalid numerical string: \"{}\" error: {}", number, e);
        }));
    }
    computer
}

fn run_program(program_text: &str, trace: bool) -> Result<Vec<i64>> {
    let mut output = VectorOutputer { vec: Vec::new() };
    let mut computer = parse_program(program_text, &mut output as &mut dyn ComputerIO);
    computer.trace = trace;
    computer.memory.trace = trace;
    while !computer.finished {
        computer.step()?;
    }
    Ok(output.vec)
}

fn part_one(input: &str) -> i32 {
    let output = run_program(input, false).unwrap();

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
    block_tiles
}

#[derive(PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    fn id_to_tile(id: i64) -> Tile {
        match id {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("Invalid tile id."),
        }
    }
}

struct Pong {
    output_buffer: Vec<i64>,
    ball: Option<(i64, i64)>,
    paddle: Option<(i64, i64)>,
    score: Option<i64>,
}

impl Pong {
    fn new() -> Self {
        Pong {
            output_buffer: Vec::new(),
            ball: None,
            paddle: None,
            score: None,
        }
    }

    fn draw(&mut self, point: (i64, i64), tile: Tile) {
        match tile {
            Tile::Ball => self.ball = Some(point),
            Tile::Paddle => self.paddle = Some(point),
            Tile::Empty | Tile::Block | Tile::Wall => {
                // Handling these is not necessary. The problem statement
                // says that we should record the score "after breaking all
                // the blocks", but apparently the intcode program tracks
                // this state for us and records a score only when it is
                // appropriate to do so.
            }
        }
    }
}

impl ComputerIO for Pong {
    fn input(&mut self) -> i64 {
        match (self.ball, self.paddle) {
            (Some(ball), Some(paddle)) => {
                // Tilt the joystick toward the ball.
                // -1 for left, 1 for right, 0 for neutral.
                (ball.0 - paddle.0).signum()
            }
            _ => panic!("invalid program state"),
        }
    }

    fn output(&mut self, value: i64) {
        self.output_buffer.push(value);
        if self.output_buffer.len() == 3 {
            let point = (self.output_buffer[0], self.output_buffer[1]);
            let arg = self.output_buffer[2];
            self.output_buffer.clear();
            if point == (-1, 0) {
                self.score = Some(arg);
            } else {
                let tile = Tile::id_to_tile(arg);
                self.draw(point, tile);
            }
        }
    }
}

fn part_two(input: &str) -> i64 {
    let mut pong = Pong::new();
    let mut computer = parse_program(input, &mut pong as &mut dyn ComputerIO);
    computer.memory.vec[0] = 2; // insert infinite quarters, per the problem instructions
    while !computer.finished {
        computer.step().unwrap();
    }
    pong.score.unwrap()
}

fn main() {
    let input = include_str!("../inputs/13.txt");
    assert_eq!(part_one(input), 277);
    assert_eq!(part_two(input), 12856);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
