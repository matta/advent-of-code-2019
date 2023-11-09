use std::{cmp::min, iter};

fn parse_digits(input: &str) -> Vec<i32> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
}

fn fft_phase(digits: &Vec<i32>) -> Vec<i32> {
    let mut next = Vec::with_capacity(digits.len());
    for stride in 1..=digits.len() {
        let mut sum = 0;

        // Handle the first use of the first value in the pattern: 0
        //
        // Because this value is zero we skip past all digits it applies to.  We
        // skip one less than all other strides because the problem statement
        // requires skipping the very first value in the sequence exactly once.
        let mut j = stride - 1;

        while j < digits.len() {
            // Handle the second value in the pattern: 1
            //
            // Because this value is one we can simply sum the digits without
            // multiplication.
            let end = min(j + stride, digits.len());
            sum += digits[j..end].iter().sum::<i32>();
            j = end;

            // Handle the third value in the pattern: 0
            //
            // Similar to the first value, these don't contribute to the sum
            // so we can simply skip them.
            j += stride;
            if j >= digits.len() {
                break;
            }

            // Handle the fourth value in the pattern: -1
            let end = min(j + stride, digits.len());
            sum += digits[j..end].iter().map(|n| -n).sum::<i32>();
            j = end;

            // Handle the first value in the pattern: 0
            j += stride;
        }
        next.push(sum.abs() % 10);
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
        assert_eq!(fft_run("87654321", 1), 48540631);
        assert_eq!(fft_run("12345678", 1), 48226158);
        assert_eq!(fft_run("12345678", 2), 34040438);
        assert_eq!(fft_run("12345678", 3), 03415518);
        assert_eq!(fft_run("12345678", 4), 01029498);
        assert_eq!(fft_run("12345678", 100), 23845678);
        assert_eq!(fft_run("80871224585914546619083218645595", 1), 24706861);
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
