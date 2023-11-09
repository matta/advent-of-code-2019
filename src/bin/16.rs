use std::iter;

fn parse_digits(input: &str) -> Vec<i32> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
}

fn fft_phase(digits: &Vec<i32>) -> Vec<i32> {
    let pattern: [i32; 4] = [1, 0, -1, 0];

    let mut next = Vec::with_capacity(digits.len());
    for i in 0..digits.len() {
        let mut result = 0;
        for (j, val) in digits.iter().enumerate().skip(i) {
            let pattern_index = (j - i) / (i + 1) % 4;
            let pattern_val = pattern[pattern_index];
            result += val * pattern_val;
        }
        let result = result.abs() % 10;
        next.push(result);
    }
    next
}

fn fft_loop(mut digits: Vec<i32>, count: u32) -> Vec<i32> {
    for _ in 0..count {
        digits = fft_phase(&digits);
    }

    digits
}

fn as_num(digits: &[i32]) -> i32 {
    digits.iter().fold(0, |acc, n| acc * 10 + n)
}

fn fft_run(input: &str, count: u32) -> i32 {
    // println!("XXX fft_run");
    let digits = fft_loop(parse_digits(input), count);
    as_num(&digits[..8])
}

fn part_one(input: &str) -> i32 {
    fft_run(input, 100)
}

fn part_two(input: &str) -> i32 {
    // Credit to all the kind people on
    // https://www.reddit.com/r/adventofcode/comments/ebai4g/2019_day_16_solutions/
    // who are more mathematically inclined and persistent than I am.
    let digits = parse_digits(input);
    let skip = as_num(&digits[..7]) as usize;
    let explode = 10000;
    assert!(skip >= digits.len() * explode / 2);
    let mut digits: Vec<i32> = iter::repeat(digits.iter().copied())
        .take(explode)
        .flatten()
        .skip(skip)
        .collect();

    for _phase in 0..100 {
        let mut sum = 0_u32;
        for i in (0..digits.len()).rev() {
            sum += digits[i] as u32;
            digits[i] = (sum % 10_u32) as i32;
        }
    }

    as_num(&digits[..8])
}

fn part_one_main() {
    let input = include_str!("../inputs/16.txt");
    assert_eq!(part_one(input), 89576828);
}

fn part_two_main() {
    let input = include_str!("../inputs/16.txt");
    assert_eq!(part_two(input), 23752579);
}

fn main() {
    part_one_main();
    part_two_main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fft_run() {
        assert_eq!(fft_run("12345678", 1), 48226158);
        assert_eq!(fft_run("12345678", 2), 34040438);
        assert_eq!(fft_run("12345678", 3), 03415518);
        assert_eq!(fft_run("12345678", 4), 01029498);
        assert_eq!(fft_run("80871224585914546619083218645595", 100), 24176176);
        assert_eq!(fft_run("19617804207202209144916044189917", 100), 73745418);
        assert_eq!(fft_run("69317163492948606335995924319873", 100), 52432133);
    }

    #[test]
    fn test_part_one() {
        part_one_main();
    }

    #[test]
    fn test_part_two() {
        part_two_main();
    }
}
