use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ops::Add;

use anyhow::bail;
use anyhow::Result;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn point(x: i32, y: i32) -> Point {
    Point { x, y }
}

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

// fn run_program(program_text: &str, input_number: i64, trace: bool) -> Result<Vec<i64>> {
//     let mut computer = parse_program(program_text);
//     computer.input = input_number;
//     computer.trace = trace;
//     computer.memory.trace = trace;
//     if trace {
//         println!("run_program: {:?}", computer);
//     }
//     computer.run()?;
//     Ok(computer.output)
// }

#[derive(Copy, Clone, Debug)]
enum Color {
    Black,
    White,
}

fn compute(input: &str, default_color: Color) -> HashMap<Point, Color> {
    let mut computer = parse_program(input);

    #[derive(Copy, Clone, Debug)]
    enum CardinalDirection {
        North,
        South,
        East,
        West,
    }

    #[derive(Copy, Clone, Debug)]
    enum Direction {
        Left,
        Right,
    }

    let mut position = point(0, 0);
    let mut direction = CardinalDirection::North;
    let mut panel: HashMap<Point, Color> = HashMap::new();

    while !computer.finished {
        let input_color = panel.get(&position).unwrap_or(&default_color);
        computer.input = match *input_color {
            Color::Black => 0,
            Color::White => 1,
        };
        computer.output.clear();
        while !computer.finished && computer.output.len() < 2 {
            computer.step().unwrap();
        }
        if computer.finished {
            break;
        }

        let (output_color, direction_change) = match computer.output[..] {
            [color_number, direction_number] => {
                let color = match color_number {
                    0 => Color::Black,
                    1 => Color::White,
                    output => panic!("Invalid program output color: {:?}", output),
                };
                let direction_change = match direction_number {
                    0 => Direction::Left,
                    1 => Direction::Right,
                    num => panic!("Invalid program output direction: {}", num),
                };

                (color, direction_change)
            }
            _ => panic!("Invalid program output: {:?}", computer.output),
        };

        match panel.entry(position) {
            Entry::Occupied(entry) => {
                *entry.into_mut() = output_color;
            }
            Entry::Vacant(entry) => {
                entry.insert(output_color);
            }
        }

        direction = match (direction, direction_change) {
            (CardinalDirection::North, Direction::Left) => CardinalDirection::West,
            (CardinalDirection::North, Direction::Right) => CardinalDirection::East,
            (CardinalDirection::East, Direction::Left) => CardinalDirection::North,
            (CardinalDirection::East, Direction::Right) => CardinalDirection::South,
            (CardinalDirection::West, Direction::Left) => CardinalDirection::South,
            (CardinalDirection::West, Direction::Right) => CardinalDirection::North,
            (CardinalDirection::South, Direction::Left) => CardinalDirection::East,
            (CardinalDirection::South, Direction::Right) => CardinalDirection::West,
        };

        position = position
            + match direction {
                CardinalDirection::North => point(0, -1),
                CardinalDirection::South => point(0, 1),
                CardinalDirection::East => point(1, 0),
                CardinalDirection::West => point(-1, 0),
            }
    }

    panel
}

fn print_panel(panel: &HashMap<Point, Color>, default_color: Color) -> String {
    let mut output = String::new();
    let print = |color: Color, output: &mut String| {
        let ch = match color {
            Color::Black => ' ',
            Color::White => '#',
        };
        output.push(ch);
    };

    let min_x = panel.keys().map(|p| p.x).min().unwrap();
    let max_x = panel.keys().map(|p| p.x).max().unwrap();
    let min_y = panel.keys().map(|p| p.y).min().unwrap();
    let max_y = panel.keys().map(|p| p.y).max().unwrap();

    for y in (min_y - 1)..=(max_y + 1) {
        for x in min_x..=max_x {
            let color = panel.get(&point(x, y)).unwrap_or(&default_color);
            print(*color, &mut output);
        }
        output.push('\n');
    }
    print!("{}", output);
    output
}

pub fn part_one(input: &str) -> usize {
    let panel = compute(input, Color::Black);
    panel.len()
}

pub fn part_two(input: &str) -> String {
    let panel = compute(input, Color::White);
    print_panel(&panel, Color::White)
}

fn main() {
    let input = include_str!("../inputs/11.txt");
    assert_eq!(part_one(input), 1564);
    assert_eq!(part_two(input),
               "###########################################\n ###  #### #### ###   ##  #### #### ###   #\n##  # #    #    #  # #  # #    #    #  #   \n##  # ###  ###  #  # #    ###  ###  ###    \n ###  #    #    ###  #    #    #    #  #  #\n # #  #    #    #    #  # #    #    #  # ##\n##  # #    #### #     ##  #    #### ###  ##\n###########################################\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
