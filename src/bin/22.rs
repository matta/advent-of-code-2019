const INPUT: &str = include_str!("../inputs/22.txt");

enum Deal {
    WithIncrement(usize),
    IntoNewStack,
    Cut(isize),
}

impl Deal {
    fn deal(&self, deck: &[i16]) -> Vec<i16> {
        match *self {
            Deal::IntoNewStack => deck.iter().rev().copied().collect(),
            Deal::WithIncrement(increment) => {
                let len = deck.len();
                let mut result: Vec<i16> = vec![0; len];
                for (i, e) in deck.iter().enumerate() {
                    result[(i * increment) % len] = *e;
                }
                result
            }
            Deal::Cut(n) => {
                let n: usize = if n >= 0 {
                    n as usize
                } else {
                    deck.len() - -n as usize
                };
                deck.iter()
                    .skip(n)
                    .chain(deck.iter().take(n))
                    .copied()
                    .collect()
            }
        }
    }
}

fn parse_deals(input: &str) -> Vec<Deal> {
    input
        .lines()
        .map(|line| {
            if line == "deal into new stack" {
                Deal::IntoNewStack
            } else if let Some(num) = line.strip_prefix("deal with increment ") {
                let num: usize = num.parse().unwrap();
                Deal::WithIncrement(num)
            } else if let Some(num) = line.strip_prefix("cut ") {
                let num: isize = num.parse().unwrap();
                Deal::Cut(num)
            } else {
                panic!("unexpected input line \"{}\"", line)
            }
        })
        .collect()
}

fn run_shuffles(deck_len: i16, input: &str) -> Vec<i16> {
    let deals: Vec<Deal> = parse_deals(input);
    let mut deck: Vec<i16> = (0..deck_len).collect();

    for deal in deals {
        deck = deal.deal(&deck);
    }

    deck
}

fn part_one() {
    let result = run_shuffles(10_007, INPUT);
    let pos = result
        .iter()
        .enumerate()
        .find(|(_, e)| **e == 2019)
        .map(|(i, _)| i);
    assert_eq!(pos, Some(1234));
}

fn main() {
    part_one();

    // Didn't do part two, which was more about math than programming.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_one() {
        let input = "\
deal with increment 7
deal into new stack
deal into new stack
";
        let result = run_shuffles(10, input);
        assert_eq!(result, vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn test_part_one_example_two() {
        let input = "\
cut 6
deal with increment 7
deal into new stack
";
        let result = run_shuffles(10, input);
        assert_eq!(result, vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn test_part_one_example_three() {
        let input = "\
deal with increment 7
deal with increment 9
cut -2
";
        let result = run_shuffles(10, input);
        assert_eq!(result, vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn test_part_one_example_four() {
        let input = "\
deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1
";
        let result = run_shuffles(10, input);
        assert_eq!(result, vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }

    #[test]
    fn test_part_one() {
        part_one();
    }
}
