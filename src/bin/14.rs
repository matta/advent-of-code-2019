use core::fmt;
use std::{cmp::max, collections::HashMap, error::Error};

#[derive(Debug, PartialEq)]
struct Chemical {
    name: u32,
    amount: u32,
}

impl fmt::Display for Chemical {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.name, self.amount)
    }
}

#[derive(Debug, PartialEq)]
struct Reaction {
    inputs: Vec<Chemical>,
    output: Chemical,
}

impl fmt::Display for Reaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sep = "";
        write!(f, "[")?;
        for input in &self.inputs {
            write!(f, "{}{}", sep, input)?;
            sep = ", ";
        }
        write!(f, "] => {}", self.output)
    }
}

struct NameTable {
    names_to_index: HashMap<String, u32>,
}

const NAME_TABLE_FUEL: u32 = 0;
const NAME_TABLE_ORE: u32 = 1;

impl NameTable {
    fn new() -> Self {
        let mut table = Self {
            names_to_index: HashMap::new(),
        };
        table.insert("FUEL".to_string());
        table.insert("ORE".to_string());
        table
    }

    fn insert(&mut self, name: String) -> u32 {
        let len = self.names_to_index.len();
        *self
            .names_to_index
            .entry(name)
            .or_insert_with(|| len as u32)
    }
}

fn parse_chemical_quantity(input: &str, names: &mut NameTable) -> Result<Chemical, Box<dyn Error>> {
    let (quantity_str, chemical) = input
        .trim()
        .split_once(' ')
        .ok_or("bad chemical quantity in input")?;
    Ok(Chemical {
        name: names.insert(chemical.to_string()),
        amount: quantity_str.parse::<u32>()?,
    })
}

fn parse_chemical_reaction(input: &str, names: &mut NameTable) -> Result<Reaction, Box<dyn Error>> {
    let (inputs_str, output_str) = input
        .trim()
        .split_once("=>")
        .ok_or("delimiter \"=>\" not found in line")?;
    let mut inputs = Vec::new();
    for input in inputs_str.split(',') {
        let chemical_quantity = parse_chemical_quantity(input, names)?;
        inputs.push(chemical_quantity);
    }
    Ok(Reaction {
        inputs,
        output: parse_chemical_quantity(output_str, names)?,
    })
}

type ReactionHash = HashMap<u32, Reaction>;

fn parse_input(input: &str) -> Result<ReactionHash, Box<dyn Error>> {
    let mut names = NameTable::new();
    let mut reactions = HashMap::new();
    for line in input.trim().lines() {
        let reaction = parse_chemical_reaction(line, &mut names)?;
        reactions.insert(reaction.output.name.clone(), reaction);
    }
    Ok(reactions)
}

// TODO use u64::div_ceil instead, if it graduates out of nightly experimental.
fn div_ceil64(x: u64, y: u64) -> u64 {
    let (q, r) = (x / y, x % y);
    if r != 0 {
        q + 1
    } else {
        q
    }
}

fn ore_required_recur(
    reactions: &ReactionHash,
    surplus: &mut HashMap<u32, u64>,
    target: u32,
    target_amount: u64,
) -> u64 {
    if target == NAME_TABLE_ORE {
        return target_amount;
    }
    let target_amount = {
        let surplus_amount = surplus.entry(target).or_insert(0);
        if target_amount <= *surplus_amount {
            *surplus_amount -= target_amount;
            return 0;
        }
        let new_target = target_amount - *surplus_amount;
        *surplus_amount = 0;
        new_target
    };

    let reaction = reactions.get(&target).unwrap();
    let copies = div_ceil64(target_amount, reaction.output.amount as u64);
    let mut ore = 0;
    for chem in reaction.inputs.iter() {
        let input_amount = copies * chem.amount as u64;
        ore += ore_required_recur(reactions, surplus, chem.name, input_amount);
    }

    let surplus_amount = surplus.entry(target.clone()).or_insert(0);
    *surplus_amount += reaction.output.amount as u64 * copies - target_amount;

    ore
}

fn ore_required(reactions: &ReactionHash, target: u32, target_amount: u64) -> u64 {
    let mut surplus = HashMap::new();
    let ore = ore_required_recur(reactions, &mut surplus, target, target_amount);
    ore
}

pub fn part_one(input: &str) -> Option<u64> {
    let reactions = parse_input(input).expect("bad input");
    Some(ore_required(&reactions, NAME_TABLE_FUEL, 1))
}

pub fn part_two(input: &str) -> Option<u64> {
    let reactions = parse_input(input).expect("bad input");

    let compute = |target_amount| ore_required(&reactions, NAME_TABLE_FUEL, target_amount);

    let target: u64 = 1_000_000_000_000;
    let mut high: u64 = 0;
    let mut low: u64 = 0;
    let mut lowest = low;
    while compute(high) <= target {
        lowest = high;
        low = high + 1;
        high = max(1, high * 8);
    }
    while low < high {
        let mid = low + (high - low) / 2;
        if compute(mid) <= target {
            lowest = mid;
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    Some(lowest)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_trivial() {
        assert_eq!(
            part_one(
                r#"7 ORE => 3 A
                   5 A => 1 FUEL"#
            ),
            Some(14)
        );
    }

    #[test]
    fn test_part_one_simple() {
        assert_eq!(
            part_one(
                r#"10 ORE => 10 A
                   1 ORE => 1 B
                   7 A, 1 B => 1 C
                   7 A, 1 C => 1 D
                   7 A, 1 D => 1 E
                   7 A, 1 E => 1 FUEL"#
            ),
            Some(31)
        );
    }

    #[test]
    fn test_part_one_simple2() {
        assert_eq!(
            part_one(
                r#"9 ORE => 2 A
                   8 ORE => 3 B
                   7 ORE => 5 C
                   3 A, 4 B => 1 AB
                   5 B, 7 C => 1 BC
                   4 C, 1 A => 1 CA
                   2 AB, 3 BC, 4 CA => 1 FUEL"#
            ),
            Some(165)
        );
    }

    #[test]
    fn test_part_one_larger() {
        let input = r#"157 ORE => 5 NZVS
                       165 ORE => 6 DCFZ
                       44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
                       12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
                       179 ORE => 7 PSHF
                       177 ORE => 5 HKGWZ
                       7 DCFZ, 7 PSHF => 2 XJWVT
                       165 ORE => 2 GPVTF
                       3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"#;

        assert_eq!(part_one(input), Some(13312));
    }

    #[test]
    fn test_part_one_larger2() {
        let input = r#"
            2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
            17 NVRVD, 3 JNWZP => 8 VPVL
            53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
            22 VJHF, 37 MNCFX => 5 FWMGM
            139 ORE => 4 NVRVD
            144 ORE => 7 JNWZP
            5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
            5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
            145 ORE => 6 MNCFX
            1 NVRVD => 8 CXFTF
            1 VJHF, 6 MNCFX => 4 RFSQX
            176 ORE => 6 VJHF"#;
        assert_eq!(part_one(input), Some(180697));
    }

    #[test]
    fn test_part_one_larger3() {
        let input = r#"
            171 ORE => 8 CNZTR
            7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
            114 ORE => 4 BHXH
            14 VRPVC => 6 BMBT
            6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
            6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
            15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
            13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
            5 BMBT => 4 WPTQ
            189 ORE => 9 KTJDG
            1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
            12 VRPVC, 27 CNZTR => 2 XDBXC
            15 KTJDG, 12 BHXH => 5 XCVML
            3 BHXH, 2 VRPVC => 7 MZWV
            121 ORE => 7 VRPVC
            7 XCVML => 6 RJRHP
            5 BHXH, 4 VRPVC => 5 LTCX
        "#;
        assert_eq!(part_one(input), Some(2210736));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), None);
    }
}
