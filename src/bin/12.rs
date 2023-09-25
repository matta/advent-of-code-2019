use itertools::Itertools;

fn parse_value(input: &str, variable: &str) -> i32 {
    match input.find(variable) {
        None => panic!("failed to find variable in \"{}\"", input),
        Some(index) => {
            let rest = &input[index + variable.len()..];
            match rest.split_once(|c| c == '>' || c == ',') {
                None => panic!("failed to parse integer in \"{}\" error: {}", rest, rest),
                Some((prefix, _)) => prefix.parse::<i32>().unwrap(),
            }
        }
    }
}

fn parse_line(input: &str) -> (i32, i32, i32) {
    let x = parse_value(input, "x=");
    let y = parse_value(input, "y=");
    let z = parse_value(input, "z=");
    (x, y, z)
}

fn parse_input(input: &str) -> Vec<(i32, i32, i32)> {
    input.trim().split('\n').map(parse_line).collect_vec()
}

#[derive(Clone, Debug)]
struct Moon {
    position: (i32, i32, i32),
    velocity: (i32, i32, i32),
}

impl Moon {
    fn apply_gravity_from(&mut self, other_pos: (i32, i32, i32)) {
        self.velocity.0 += (other_pos.0 - self.position.0).signum();
        self.velocity.1 += (other_pos.1 - self.position.1).signum();
        self.velocity.2 += (other_pos.2 - self.position.2).signum();
    }

    fn total_energy(&self) -> i32 {
        (self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs())
            * (self.position.0.abs() + self.position.1.abs() + self.position.2.abs())
    }
}

fn compute_part_one(input: &str, steps: i32) -> i32 {
    let mut moons = parse_input(input)
        .into_iter()
        .map(|position| Moon {
            position,
            velocity: (0, 0, 0),
        })
        .collect_vec();

    for _ in 0..steps {
        // println!("After {} steps:", i);
        // for moon in moons.iter() {
        // println!("{:?}", moon);
        // }

        // Apply gravity.
        for (i, j) in (0..moons.len()).tuple_combinations() {
            let mut other_pos = moons[j].position;
            moons[i].apply_gravity_from(other_pos);

            other_pos = moons[i].position;
            moons[j].apply_gravity_from(other_pos);
        }

        // Apply velocities.
        for moon in moons.iter_mut() {
            moon.position.0 += moon.velocity.0;
            moon.position.1 += moon.velocity.1;
            moon.position.2 += moon.velocity.2;
        }
    }

    moons.iter().map(|moon| moon.total_energy()).sum()
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(compute_part_one(input, 1000))
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = r#"
        <x=-1, y=0, z=2>
        <x=2, y=-10, z=-7>
        <x=4, y=-8, z=8>
        <x=3, y=5, z=-1>
    "#;

    static EXAMPLE2: &str = r#"
        <x=-8, y=-10, z=0>
        <x=5, y=5, z=10>
        <x=2, y=-7, z=3>
        <x=9, y=-8, z=-3>
    "#;

    #[test]
    fn test_parse_value() {
        let str = "<x=-1, y=0, z=2>";
        assert_eq!(parse_value(str, "x="), -1);
        assert_eq!(parse_value(str, "y="), 0);
        assert_eq!(parse_value(str, "z="), 2);
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(EXAMPLE1),
            vec![(-1, 0, 2), (2, -10, -7), (4, -8, 8), (3, 5, -1)]
        )
    }

    #[test]
    fn test_apply_gravity() {
        let mut ganymede = Moon {
            position: (3, 2, 1),
            velocity: (0, 0, 0),
        };
        let mut callisto = Moon {
            position: (1, 2, 3),
            velocity: (0, 0, 0),
        };
        ganymede.apply_gravity_from(callisto.position);
        callisto.apply_gravity_from(ganymede.position);

        assert_eq!(ganymede.position, (3, 2, 1));
        assert_eq!(ganymede.velocity, (-1, 0, 1));

        assert_eq!(callisto.position, (1, 2, 3));
        assert_eq!(callisto.velocity, (1, 0, -1));
    }

    #[test]
    fn test_compute_part_one_a() {
        assert_eq!(compute_part_one(EXAMPLE1, 10), 179)
    }

    #[test]
    fn test_compute_part_one_b() {
        assert_eq!(compute_part_one(EXAMPLE2, 100), 1940)
    }
}
