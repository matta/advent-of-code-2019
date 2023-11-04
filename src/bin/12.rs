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

#[derive(Clone, Copy, Debug, PartialEq)]
struct MoonAxis {
    position: i32,
    velocity: i32,
}

impl MoonAxis {
    fn apply_gravity_from(&mut self, other: MoonAxis) {
        self.velocity += (other.position - self.position).signum();
    }

    fn apply_velocity(&mut self) {
        self.position += self.velocity
    }
}

#[derive(Clone, Debug)]
struct Moon {
    x: MoonAxis,
    y: MoonAxis,
    z: MoonAxis,
}

impl Moon {
    fn apply_gravity_from(&mut self, other: &Moon) {
        self.x.apply_gravity_from(other.x);
        self.y.apply_gravity_from(other.y);
        self.z.apply_gravity_from(other.z);
    }

    fn apply_velocity(&mut self) {
        self.x.apply_velocity();
        self.y.apply_velocity();
        self.z.apply_velocity();
    }

    fn total_energy(&self) -> i32 {
        (self.x.position.abs() + self.y.position.abs() + self.z.position.abs())
            * (self.x.velocity.abs() + self.y.velocity.abs() + self.z.velocity.abs())
    }
}

fn parse_moons(input: &str) -> Vec<Moon> {
    parse_input(input)
        .into_iter()
        .map(|position| Moon {
            x: MoonAxis {
                position: position.0,
                velocity: 0,
            },
            y: MoonAxis {
                position: position.1,
                velocity: 0,
            },
            z: MoonAxis {
                position: position.2,
                velocity: 0,
            },
        })
        .collect_vec()
}

fn compute_part_one(input: &str, steps: i32) -> i32 {
    let mut moons = parse_moons(input);

    for _ in 0..steps {
        // println!("After {} steps:", i);
        // for moon in moons.iter() {
        // println!("{:?}", moon);
        // }

        // Apply gravity.
        for i in 0..(moons.len() - 1) {
            for j in (i + 1)..moons.len() {
                let (left, right) = moons.split_at_mut(j);
                left[i].apply_gravity_from(&right[0]);
                right[0].apply_gravity_from(&left[i]);
            }
        }

        // Apply velocities.
        for moon in moons.iter_mut() {
            moon.apply_velocity();
        }
    }

    moons.iter().map(|moon| moon.total_energy()).sum()
}

fn calculate_gravity(axes: &mut [MoonAxis]) {
    for i in 0..axes.len() - 1 {
        for j in i + 1..axes.len() {
            let (left_axes, right_axes) = axes.split_at_mut(j);
            left_axes[i].apply_gravity_from(right_axes[0]);
            right_axes[0].apply_gravity_from(left_axes[i]);
        }
    }
}

fn apply_velocities(axes: &mut [MoonAxis]) {
    for axis in axes.iter_mut() {
        axis.apply_velocity();
    }
}

fn find_period(axes: &[MoonAxis]) -> i64 {
    let mut period = 0;
    let mut curr = axes.to_vec();
    loop {
        period += 1;
        calculate_gravity(&mut curr);
        apply_velocities(&mut curr);
        if *curr == *axes {
            break;
        }
    }
    period
}

fn get_axes<F, T>(moons: &[Moon], get: F) -> Vec<T>
where
    F: Fn(&Moon) -> T,
{
    moons.iter().map(get).collect()
}

fn gcd(mut m: i64, mut n: i64) -> i64 {
    while m != 0 {
        let old_m = m;
        m = n % m;
        n = old_m;
    }

    n
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn compute_part_two(input: &str) -> i64 {
    let moons = parse_moons(input);

    let x_period = find_period(&get_axes(&moons, |axis| axis.x));
    let y_period = find_period(&get_axes(&moons, |axis| axis.y));
    let z_period = find_period(&get_axes(&moons, |axis| axis.z));

    lcm(x_period, lcm(y_period, z_period))
}

pub fn part_one(input: &str) -> i32 {
    compute_part_one(input, 1000)
}

pub fn part_two(_input: &str) -> i64 {
    compute_part_two(_input)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    assert_eq!(part_one(input), 14780);
    assert_eq!(part_two(input), 279751820342592);
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
    fn test_compute_part_one_a() {
        assert_eq!(compute_part_one(EXAMPLE1, 10), 179)
    }

    #[test]
    fn test_compute_part_one_b() {
        assert_eq!(compute_part_one(EXAMPLE2, 100), 1940)
    }

    #[test]
    fn test_compute_part_two_example_1() {
        assert_eq!(compute_part_two(EXAMPLE1), 2772)
    }

    #[test]
    fn test_compute_part_two_example_2() {
        assert_eq!(compute_part_two(EXAMPLE2), 4686774924)
    }

    #[test]
    fn test_main() {
        main();
    }
}
