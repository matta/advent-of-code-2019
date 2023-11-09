use std::cmp::min;

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
    // Credit to all the kind people on Reddit who are better than I am at
    // figuring out these kinds of problems:
    //
    // https://www.reddit.com/r/adventofcode/comments/ebai4g/2019_day_16_solutions/
    // https://www.reddit.com/r/adventofcode/comments/ebf5cy/2019_day_16_part_2_understanding_how_to_come_up/
    // https://www.reddit.com/r/adventofcode/comments/ebai4g/comment/fb3kujj/?utm_source=share&utm_medium=web2x&context=3
    //
    // From https://www.reddit.com/r/adventofcode/comments/ebf5cy/comment/fb5p0hy
    // we have this:
    //
    // I think the best way to understand the solution is just to write out
    // the operations for the example from part 1. Then, the pattern is
    // obviously a backwards cumulative sum % 10.
    //
    // Input signal: 12345678
    //
    // 1*1 + ... + 8*0 = 4
    // 1*0 + ... + 8*0 = 8
    // 1*0 + ... + 8*0 = 2
    // 1*0 + ... + 8*0 = 2
    // 1*0 + ... + 8*1 = 6 = (8 + 7 + 6 + 5) % 10
    // 1*0 + ... + 8*1 = 1 = (8 + 7 + 6) % 10
    // 1*0 + ... + 8*1 = 5 = (8 + 7) % 10
    // 1*0 + ... + 8*1 = 8 = (8) % 10
    //
    // So, we exploit the pattern that the result at pos i past the midway
    // point is the cumulative sum of each digit (i+1).. mod 10.

    let mut suffix = {
        let digits = parse_digits(input);
        let exploded_len = digits.len() * 10_000;
        let skip: usize = as_num(&digits[..7]).try_into().unwrap();
        // Insist that we need compute the FFT for somewhere in the
        // second half of the signal.
        assert!(skip >= exploded_len / 2);
        digits
            .iter()
            .copied()
            .cycle()
            .take(exploded_len)
            .skip(skip)
            .collect::<Vec<i32>>()
    };

    for _ in 0..100 {
        let mut sum = 0;
        for value in suffix.iter_mut().rev() {
            sum += *value;
            *value = sum % 10;
        }
    }

    as_num(&suffix[..8])
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
