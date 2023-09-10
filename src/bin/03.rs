#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    value: i32,
}

fn parse_input(input: &str) -> Vec<Vec<Instruction>> {
    let mut wires = Vec::new();

    input.trim_end().split('\n').for_each(|line| {
        let mut instructions: Vec<Instruction> = Vec::new();
        line.split(',').for_each(|instruction| {
            let (direction, value) = instruction.split_at(1);
            let value: i32 = value.parse().unwrap_or_else(|e| {
                panic!("Invalid string: \"{}\" error: {}", value, e);
            });
            let direction = direction.chars().next().unwrap();
            let direction = match direction {
                'R' => Direction::Right,
                'L' => Direction::Left,
                'U' => Direction::Up,
                'D' => Direction::Down,
                _ => panic!("Invalid direction: {}", direction),
            };
            instructions.push(Instruction { direction, value });
        });
        wires.push(instructions);
    });

    wires
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn manhattan_distance_from_zero(self) -> u32 {
        self.x.unsigned_abs() + self.y.unsigned_abs()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let wires = parse_input(input);

    type Visited = [bool; 2];
    type VisitedPoints = std::collections::HashMap<Point, Visited>;

    // Iterate over the instructions and populate the visited points.
    let mut visited_points = VisitedPoints::new();
    for (index, instructions) in wires.iter().enumerate() {
        let mut current_point = Point { x: 0, y: 0 };
        for instruction in instructions {
            let incr_point = match instruction.direction {
                Direction::Right => Point { x: 1, y: 0 },
                Direction::Left => Point { x: -1, y: 0 },
                Direction::Up => Point { x: 0, y: -1 },
                Direction::Down => Point { x: 0, y: 1 },
            };
            for _ in 0..instruction.value {
                current_point = current_point.add(incr_point);
                let entry = visited_points
                    .entry(current_point)
                    .or_insert([false, false]);
                entry[index] = true;
            }
        }
    }

    let mut central_point = None;
    visited_points.iter().for_each(|(point, visited)| {
        if visited[0] && visited[1] {
            match central_point {
                None => central_point = Some(point),
                Some(p) => {
                    if p.manhattan_distance_from_zero() > point.manhattan_distance_from_zero() {
                        central_point = Some(point);
                    }
                }
            };
        }
    });

    Some(central_point.unwrap().manhattan_distance_from_zero())
}

pub fn part_two(input: &str) -> Option<u32> {
    let wires = parse_input(input);

    type Visited = [u32; 2];
    type VisitedPoints = std::collections::HashMap<Point, Visited>;

    // Iterate over the instructions and populate the visited points.
    let mut visited_points = VisitedPoints::new();
    for (index, instructions) in wires.iter().enumerate() {
        let mut current_point = Point { x: 0, y: 0 };
        let mut distance = 0;
        for instruction in instructions {
            let incr_point = match instruction.direction {
                Direction::Right => Point { x: 1, y: 0 },
                Direction::Left => Point { x: -1, y: 0 },
                Direction::Up => Point { x: 0, y: -1 },
                Direction::Down => Point { x: 0, y: 1 },
            };
            for _ in 0..instruction.value {
                current_point = current_point.add(incr_point);
                distance += 1;
                let entry = visited_points.entry(current_point).or_insert([0, 0]);
                if entry[index] == 0 {
                    entry[index] = distance;
                }
            }
        }
    }

    let mut fewest_combined_steps = None;
    for steps in visited_points.values() {
        if steps[0] > 0 && steps[1] > 0 {
            let combined_steps = steps[0] + steps[1];
            if fewest_combined_steps.unwrap_or(u32::MAX) >= combined_steps {
                fewest_combined_steps = Some(combined_steps);
            }
        }
    }

    fewest_combined_steps
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83\n"),
            Some(159)
        );
        assert_eq!(
            part_one("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7\n"),
            Some(135)
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83\n"),
            Some(610)
        );
        assert_eq!(
            part_two("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7\n"),
            Some(410)
        );
    }
}
