use std::iter;

fn parse_digits(input: &str) -> Vec<u8> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn fft_phase(phase: u32, digits: &Vec<u8>) -> Vec<u8> {
    let base_pattern: [i32; 4] = [0, 1, 0, -1];

    (1..=digits.len())
        .map(|index| {
            if index % 10000 == 0 {
                println!(
                    "phase {} index {} {:02}%",
                    phase,
                    index,
                    index as f32 / digits.len() as f32 * 100.0
                );
            }
            let pattern = base_pattern
                .iter()
                .flat_map(|p| iter::repeat(*p).take(index))
                .cycle()
                .skip(1);

            let product = digits
                .iter()
                .zip(pattern)
                .map(|(digit, base)| *digit as i32 * base)
                .sum::<i32>();

            (product.abs() % 10) as u8
        })
        .collect()
}

fn fft_loop(mut digits: Vec<u8>, count: u32) -> Vec<u8> {
    for phase in 0..count {
        digits = fft_phase(phase, &digits);
    }

    digits
}

fn as_num(digits: &[u8]) -> i32 {
    digits.iter().fold(0, |acc, n| {
        let n: i32 = (*n).into();
        acc * 10 + n
    })
}

fn fft_run(input: &str, count: u32) -> i32 {
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
    let mut digits: Vec<u8> = iter::repeat(digits.iter().copied())
        .take(explode)
        .flatten()
        .skip(skip)
        .collect();

    for _phase in 0..100 {
        let mut sum = 0_u32;
        for i in (0..digits.len()).rev() {
            sum += digits[i] as u32;
            digits[i] = (sum % 10_u32) as u8;
        }
    }

    as_num(&digits[..8])
}

fn main() {
    let input = include_str!("../inputs/16.txt");
    assert_eq!(part_one(input), 89576828);
    assert_eq!(part_two(input), 23752579);
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
    fn test_main() {
        main()
    }
}
