fn split_string_by_dash(string: &str) -> (Vec<u8>, Vec<u8>) {
    let (first_part, second_part) = string
        .rsplit_once("-")
        .unwrap_or_else(|| panic!("The string does not contain a dash"));
    (
        first_part.as_bytes().to_vec(),
        second_part.as_bytes().to_vec(),
    )
}

fn lexicographically_increment_vec(vec: &mut Vec<u8>) {
    let mut i = vec.len();

    while i > 0 {
        i -= 1;
        if vec[i] < b'9' {
            vec[i] += 1;
            return;
        }

        vec[i] = b'0';
    }

    vec.push(b'1');
}

fn is_lexicographically_le(vec1: &Vec<u8>, vec2: &Vec<u8>) -> bool {
    match vec1.cmp(vec2) {
        std::cmp::Ordering::Less | std::cmp::Ordering::Equal => true,
        std::cmp::Ordering::Greater => false,
    }
}

fn is_valid_password(vec: &[u8]) -> bool {
    if vec.len() != 6 {
        return false;
    }
    let mut have_consecutive_digits = false;
    for i in 0..vec.len() - 1 {
        match vec[i].cmp(&vec[i + 1]) {
            std::cmp::Ordering::Greater => return false,
            std::cmp::Ordering::Equal => {
                have_consecutive_digits = true;
            }
            std::cmp::Ordering::Less => {}
        }
    }

    have_consecutive_digits
}

pub fn part_one(input: &str) -> Option<u32> {
    let (begin, end) = split_string_by_dash(input);
    let mut curr = begin;
    let mut valid_password_count = 0;
    while is_lexicographically_le(&curr, &end) {
        if is_valid_password(&curr) {
            valid_password_count += 1
        }
        lexicographically_increment_vec(&mut curr);
    }
    println!("valid_password_count: {}", valid_password_count);
    Some(valid_password_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("111111-111111"), Some(1));
        assert_eq!(part_one("223450-223450"), Some(0));
        assert_eq!(part_one("123789-123789"), Some(0));
        assert_eq!(part_one("123455-123466"), Some(2));
    }

    // #[test]
    // fn test_part_two() {
    //     let input = advent_of_code::read_file("examples", 4);
    //     assert_eq!(part_two(&input), None);
    // }
}
