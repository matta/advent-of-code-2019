use std::{cmp::Ordering, fmt};

use aoc2019::intcode::{Computer, RunState};
use rand::{distributions::Standard, prelude::Distribution, rngs::ThreadRng, Rng};

const INTCODE_PROGRAM: &str = include_str!("../inputs/21.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ReadableRegister {
    T,
    J,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
}

impl Distribution<ReadableRegister> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ReadableRegister {
        match rng.gen_range(0..11) {
            0 => ReadableRegister::T,
            1 => ReadableRegister::J,
            2 => ReadableRegister::A,
            3 => ReadableRegister::B,
            4 => ReadableRegister::C,
            5 => ReadableRegister::D,
            6 => ReadableRegister::E,
            7 => ReadableRegister::F,
            8 => ReadableRegister::G,
            9 => ReadableRegister::H,
            10 => ReadableRegister::I,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for ReadableRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            ReadableRegister::T => "T",
            ReadableRegister::J => "J",
            ReadableRegister::A => "A",
            ReadableRegister::B => "B",
            ReadableRegister::C => "C",
            ReadableRegister::D => "D",
            ReadableRegister::E => "E",
            ReadableRegister::F => "F",
            ReadableRegister::G => "G",
            ReadableRegister::H => "H",
            ReadableRegister::I => "I",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum WritableRegister {
    T,
    J,
}

impl fmt::Display for WritableRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            WritableRegister::T => "T",
            WritableRegister::J => "J",
        };
        write!(f, "{}", s)
    }
}

impl Distribution<WritableRegister> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> WritableRegister {
        match rng.gen_range(0..2) {
            0 => WritableRegister::T,
            1 => WritableRegister::J,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Opcode {
    Not,
    And,
    Or,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Opcode::Not => "NOT",
            Opcode::And => "AND",
            Opcode::Or => "OR",
        };
        write!(f, "{}", s)
    }
}

impl Distribution<Opcode> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Opcode {
        match rng.gen_range(0..2) {
            0 => Opcode::Not,
            1 => Opcode::And,
            2 => Opcode::Or,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Instruction {
    op: Opcode,
    read: ReadableRegister,
    write: WritableRegister,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.op, self.read, self.write)
    }
}

impl Distribution<Instruction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Instruction {
        Instruction {
            op: rng.gen(),
            read: rng.gen(),
            write: rng.gen(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Script {
    instructions: Vec<Instruction>,
}
const SCRIPT_INSTRUCTIONS_LEN_MAX: usize = 15;

impl fmt::Display for Script {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in &self.instructions {
            writeln!(f, "{}", i)?
        }
        Ok(())
    }
}

impl Distribution<Script> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Script {
        Script {
            instructions: (0..(rng.gen_range(5..10)))
                .map(|_| {
                    let inst: Instruction = rng.gen();
                    inst
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EvolvingScript {
    script: Script,
    efficacy: usize,
}

impl Ord for EvolvingScript {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.efficacy, self.script.instructions.len(), &self.script).cmp(&(
            self.efficacy,
            other.script.instructions.len(),
            &other.script,
        ))
    }
}

impl PartialOrd for EvolvingScript {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy)]
enum Speed {
    Walk,
    Run,
}

fn print_output(output: &str, speed: Speed) {
    let separator = "\n\n";
    let mut bot_sensors = None;
    let mut has_jumped = false;
    for paragraph in output.split(separator) {
        if !paragraph.contains("..............") {
            print!("{}{}", paragraph, separator);
        } else {
            let lines: Vec<&str> = paragraph.lines().collect();
            let ground = lines[2];
            if bot_sensors.is_none() || !has_jumped {
                if let Some(pos) = ground.chars().position(|c| c == '@') {
                    let mut sensors = String::new();
                    for _ in 0..=pos {
                        sensors.push(' ');
                    }
                    sensors.push_str(match speed {
                        Speed::Walk => "ABCD",
                        Speed::Run => "ABCDEFGHI",
                    });
                    bot_sensors = Some(sensors);
                } else {
                    has_jumped = true;
                }
            }
            println!("{}", paragraph);
            if let Some(sensors) = &bot_sensors {
                println!("{}", sensors);
            }
            println!();
        }
    }
}

fn check_case(script: &Script, case: &[bool]) -> bool {
    let mut i = 0;
    while i < case.len() {
        if !case[i] {
            return false; // fell in a hole
        }
        let (mut t, mut j) = (false, false);
        for instr in &script.instructions {
            let read_arg = match instr.read {
                ReadableRegister::T => t,
                ReadableRegister::J => j,
                ReadableRegister::A => case.get(i + 1).copied().unwrap_or(true),
                ReadableRegister::B => case.get(i + 2).copied().unwrap_or(true),
                ReadableRegister::C => case.get(i + 3).copied().unwrap_or(true),
                ReadableRegister::D => case.get(i + 4).copied().unwrap_or(true),
                ReadableRegister::E => case.get(i + 5).copied().unwrap_or(true),
                ReadableRegister::F => case.get(i + 6).copied().unwrap_or(true),
                ReadableRegister::G => case.get(i + 7).copied().unwrap_or(true),
                ReadableRegister::H => case.get(i + 8).copied().unwrap_or(true),
                ReadableRegister::I => case.get(i + 9).copied().unwrap_or(true),
            };
            match (instr.op, instr.write) {
                (Opcode::Not, WritableRegister::T) => t = !read_arg,
                (Opcode::Not, WritableRegister::J) => j = !read_arg,
                (Opcode::And, WritableRegister::T) => t = t && read_arg,
                (Opcode::And, WritableRegister::J) => j = j && read_arg,
                (Opcode::Or, WritableRegister::T) => t = t || read_arg,
                (Opcode::Or, WritableRegister::J) => j = j || read_arg,
            }
        }
        i += if j { 4 } else { 1 }
    }
    true
}

fn check_cases(cases: &[Vec<bool>], script: &mut EvolvingScript) {
    let efficacy = cases
        .iter()
        .filter(|case| check_case(&script.script, case))
        .count();

    let efficacy_improved = efficacy > script.efficacy;
    script.efficacy = efficacy;

    // If a mutation increased the script's efficacy perhaps it can be
    // minimized.
    if efficacy_improved {
        // Try deleting each instruction in turn.
        let mut i = 0;
        while i < script.script.instructions.len() {
            let mut script_copy = script.clone();
            script_copy.script.instructions.remove(i);
            check_cases(cases, &mut script_copy);
            if script_copy.efficacy == script.efficacy {
                *script = script_copy;
            } else {
                i += 1;
            }
        }
    }
}

// Return the number of test cases the supplied script passes, along with
// the script itself.
fn evaluate(cases: &[Vec<bool>], speed: Speed, script: &mut EvolvingScript) {
    check_cases(cases, script);
    if script.efficacy < cases.len() {
        return;
    }

    // See https://github.com/birkenfeld/advent/blob/master/2019/src/bin/2019_day21.rs
    let script_text = format!("{}", script.script);
    intcode_interpret_springscript(&script_text, speed);
    println!("Successful script:\n{}\n", script_text);
    todo!("gracefully terminate here...");
}

fn mutate(script: &mut EvolvingScript) {
    let mut rng = rand::thread_rng();
    let instructions = &mut script.script.instructions;
    loop {
        match rng.gen_range(0..=3) {
            0 => {
                let len = instructions.len();
                let max_inserts = SCRIPT_INSTRUCTIONS_LEN_MAX - len;
                if max_inserts >= 2 {
                    let inserts = rng.gen_range(2..=max_inserts);
                    let pos = rng.gen_range(0..=len);
                    instructions.splice(
                        pos..pos,
                        (0..inserts).map(|_| {
                            let inst: Instruction = rng.gen();
                            inst
                        }),
                    );
                    break;
                }
            }
            1 => {
                // Completely replace one instruction with a random one.
                let len = instructions.len();
                instructions[rng.gen_range(0..len)] = rng.gen();
                break;
            }
            2 => {
                // Remove a random instruction.
                let len = instructions.len();
                if len > 1 {
                    instructions.remove(rng.gen_range(0..len));
                    break;
                }
            }
            3 => {
                // Add an instruction at a random position.
                let len = instructions.len();
                if len < SCRIPT_INSTRUCTIONS_LEN_MAX {
                    instructions.insert(rng.gen_range(0..=len), rng.gen());
                    break;
                }
            }
            _ => unreachable!(),
        }
    }
}

fn generate(cases: &[Vec<bool>]) {
    let population_seed_count = 3_000;
    let population_len = population_seed_count * 10;

    let mut population: Vec<EvolvingScript> = Vec::new();

    let mut rng = rand::thread_rng();

    let random_script = |rng: &mut ThreadRng| {
        let script: Script = rng.gen();
        script
    };

    for generation in 1.. {
        population.truncate(population_seed_count);
        let seed_count = population.len();
        population.extend((0..population_seed_count).map(|_| EvolvingScript {
            script: random_script(&mut rng),
            efficacy: 0,
        }));
        let mut seed_index = 0;
        while population.len() < population_len {
            let mut script = population[seed_index].clone();
            mutate(&mut script);
            population.push(script);
            seed_index += 1;
            if seed_index == seed_count {
                seed_index = 0;
            }
        }

        for script in population.iter_mut().skip(seed_count) {
            evaluate(cases, Speed::Run, script);
        }

        population.sort_unstable();
        population.dedup();

        if generation % 10 == 0 {
            println!(
                "Generation {} population size {}",
                generation,
                population.len()
            );
            let first = population.first().unwrap();
            let last_seed = population[seed_count - 1].clone();
            let last = population.last().unwrap();
            println!(
                "first efficacy {} len {}",
                first.efficacy,
                first.script.instructions.len()
            );
            println!(
                "last seed efficacy {} len {}",
                last_seed.efficacy,
                last_seed.script.instructions.len()
            );
            println!(
                "last efficacy {} len {}",
                last.efficacy,
                last.script.instructions.len()
            );
            println!(
                "Generation {} best efficacy {} len {}",
                generation,
                population[0].efficacy,
                population[0].script.instructions.len(),
            );
            println!("{}", population[0].script);
        }

        let best = population[0].efficacy;
        if best == cases.len() {
            println!("Found script that passes all cases:");
            println!("{}", population[0].script);
            return;
        }
    }
}

fn parse_case(case: &str) -> Vec<bool> {
    case.chars().map(|ch| ch == '#').collect()
}

fn parse_cases(cases: &[&str]) -> Vec<Vec<bool>> {
    cases.iter().map(|case| parse_case(case)).collect()
}

fn intcode_interpret_springscript(program: &str, speed: Speed) -> i64 {
    let mut computer = Computer::parse(INTCODE_PROGRAM);

    loop {
        match computer.run() {
            RunState::BlockedOnInput => {
                computer.append_str(program);
                computer.append_str(match speed {
                    Speed::Walk => "WALK\n",
                    Speed::Run => "RUN\n",
                });
            }
            RunState::BlockedOnOutput => {
                if let Some(str) = computer.read_ascii_string() {
                    print_output(&str, speed);
                } else {
                    return computer.take_output().unwrap();
                }
            }
            RunState::Finished => {
                panic!("springscript failed: program:\n{}", program);
            }
        }
    }
}

fn main() {
    // This list is manually crated from failures printed by the generate
    // function.
    let cases = parse_cases(&[
        "#####.###########",
        "#####..#.########",
        "#####...#########",
        "#####.#..########",
        "#####..#####..###",
        "#####.#.#.##.####",
        "#####.##.##..####",
        "#####.#.###.#.###",
        "#####.#.#..##.###",
        "#####..####...###",
        "#####.####...####",
        "#####.##.##.#.###",
        "#####.##...#..###",
        "#####.#.#.#..####",
    ]);
    generate(&cases);

    // Avoid dead code warning by referencing the tests.
    if false {
        test_answers();
    }
}

// (~A | ~B | ~C) & D
// ~(A & B & C) & D
const PART_ONE_SPRINGSCRIPT: &str = "\
OR A J
AND B J
AND C J
NOT J J
AND D J
";

const PART_TWO_SPRINGSCRIPT: &str = "\
NOT C J
AND H J
NOT J J
AND B J
AND A J
NOT J J
AND D J
";

fn test_answers() {
    assert_eq!(
        intcode_interpret_springscript(PART_ONE_SPRINGSCRIPT, Speed::Walk),
        19352638
    );
    assert_eq!(
        intcode_interpret_springscript(PART_TWO_SPRINGSCRIPT, Speed::Run),
        1141251258
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        test_answers();
    }
}
