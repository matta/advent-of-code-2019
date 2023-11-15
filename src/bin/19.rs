use rand::prelude::*;
use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
};

use aoc2019::{
    intcode::{Computer, RunState},
    point::Point2D,
};

const INPUT: &str = include_str!("../inputs/19.txt");

const PART_ONE_RANGE: Range<i64> = 0..50;

type Point = Point2D<i64>;

struct BeamProber {
    computer: Computer,
    probes: HashMap<Point, bool>,
    hits: i64,
}

impl BeamProber {
    fn new(intcode: &str) -> Self {
        Self {
            computer: Computer::parse(intcode),
            probes: HashMap::new(),
            hits: 0,
        }
    }

    fn engaged(&mut self, point: &Point) -> bool {
        if let Some(result) = self.probes.get(point) {
            println!("engaged? {} -> {} (cached)", point, *result);
            return *result;
        }
        // Uncharacteristically, this intcode computer is a one-shot.
        // Running the program once puturbs memory and makes it unusable
        // for subsequent runs.  Address this by cloning the computer
        // before
        let mut computer = self.computer.clone();
        computer.append_input(&[point.x, point.y]);
        let result = match computer.run() {
            RunState::BlockedOnOutput(0) => false,
            RunState::BlockedOnOutput(1) => true,
            run_result => panic!("unexpected result: {:?}", run_result),
        };
        if result {
            self.hits += 1;
        }
        self.probes.insert(*point, result);
        println!("engaged? {} -> {}", point, result);
        result
    }

    fn seen(&self, point: &Point) -> bool {
        self.probes.contains_key(point)
    }

    fn hits(&self) -> i64 {
        self.hits
    }
}

fn part_one(prober: &mut BeamProber) {
    let mut rng = thread_rng();

    prober.engaged(&Point::default());

    let mut queue = VecDeque::new();
    while queue.is_empty() {
        let point = Point::new(rng.gen_range(PART_ONE_RANGE), rng.gen_range(PART_ONE_RANGE));
        if !prober.seen(&point) && prober.engaged(&point) {
            queue.push_back(point);
        }
    }

    while let Some(point) = queue.pop_front() {
        if prober.engaged(&point) {
            for neighbor in point.neighbors() {
                if PART_ONE_RANGE.contains(&neighbor.x)
                    && PART_ONE_RANGE.contains(&neighbor.y)
                    && !prober.seen(&neighbor)
                {
                    queue.push_front(neighbor);
                }
            }
        }
    }
    let part_one_answer = prober.hits();
    assert_eq!(part_one_answer, 150);
}

fn part_two(prober: &mut BeamProber) {
    let low_intercept = {
        let bottom_row = PART_ONE_RANGE.map(|x| Point::new(x, PART_ONE_RANGE.end - 1));
        let right_column = PART_ONE_RANGE
            .rev()
            .skip(1)
            .map(|y| Point::new(PART_ONE_RANGE.end - 1, y));
        bottom_row
            .chain(right_column)
            .find(|point| prober.engaged(point))
            .unwrap()
    };

    println!("low_intercept: {}", low_intercept);

    let mut slope = low_intercept.x as f64 / low_intercept.y as f64;
    println!("slope: {:3}", slope);

    let mut find_intercept = |prober: &mut BeamProber, x: i64| {
        let mut y = (x as f64 / slope) as i64;
        if prober.engaged(&Point::new(x, y)) {
            while prober.engaged(&Point::new(x, y + 1)) {
                y += 1;
            }
        } else {
            y -= 1;
            while !prober.engaged(&Point::new(x, y)) {
                y -= 1;
            }
        }
        let new_slope = (x as f64) / (y as f64);
        let slope_delta = new_slope - slope;
        slope = new_slope;
        println!("\t({:3}, {:3})\t{:.4} ({:.5})", x, y, slope, slope_delta);
        y
    };

    let square_fits = |prober: &mut BeamProber, bottom_left: &Point| -> bool {
        let top_right = *bottom_left + Point::new(99, -99);
        top_right.y >= 0 && prober.engaged(&top_right)
    };

    let mut low_x = low_intercept.x;
    let mut high_x = None;
    for x in (1..).map(|p| 2i64.pow(p) * low_intercept.x) {
        let y = find_intercept(prober, x);
        if square_fits(prober, &Point::new(x, y)) {
            high_x = Some(x);
            break;
        } else {
            low_x = x;
        }
    }
    let mut high_x = high_x.unwrap();
    println!("high_x {:?}", high_x);

    while low_x < high_x {
        println!("low {} high {}", low_x, high_x);
        let x = (high_x + low_x) / 2;
        let y = find_intercept(prober, x);
        if square_fits(prober, &Point::new(x, y)) {
            high_x = x;
        } else {
            low_x = x + 1;
        }
    }
    println!("low {} high {}", low_x, high_x);

    let upper_left = Point::new(high_x, find_intercept(prober, high_x) - 99);
    let part_two_answer = upper_left.x * 10_000 + upper_left.y;
    assert_eq!(part_two_answer, 12201460);
}

fn main() {
    let mut prober = BeamProber::new(INPUT);
    part_one(&mut prober);
    part_two(&mut prober);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        part_one(&mut BeamProber::new(INPUT));
    }

    #[test]
    fn test_part_two() {
        part_two(&mut BeamProber::new(INPUT));
    }

    #[test]
    fn test_main() {
        main();
    }
}
