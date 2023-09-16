fn split_string_by_dash(string: &str) -> (Vec<u8>, Vec<u8>) {
    let (first_part, second_part) = string
        .rsplit_once('-')
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

fn is_valid_part_one_password(vec: &Vec<u8>) -> bool {
    if vec.len() != 6 {
        return false;
    }
    let mut has_consecutive_digits = false;
    for i in 0..vec.len() - 1 {
        match vec[i].cmp(&vec[i + 1]) {
            std::cmp::Ordering::Greater => return false,
            std::cmp::Ordering::Equal => {
                has_consecutive_digits = true;
            }
            std::cmp::Ordering::Less => {}
        }
    }

    has_consecutive_digits
}

fn is_valid_part_two_password(password: &Vec<u8>) -> bool {
    if password.len() != 6 {
        return false;
    }
    let mut has_two_consecutive_digits = false;
    for i in 0..password.len() - 1 {
        match password[i].cmp(&password[i + 1]) {
            std::cmp::Ordering::Greater => return false,
            std::cmp::Ordering::Equal => {
                // Reject runs of more than two digits.
                if i > 0 && password[i - 1] == password[i] {
                    continue;
                }
                if i < password.len() - 2 && password[i + 2] == password[i] {
                    continue;
                }
                has_two_consecutive_digits = true;
            }
            std::cmp::Ordering::Less => {}
        }
    }

    has_two_consecutive_digits
}

pub fn compute<F>(input: &str, is_valid_password: F) -> Option<u32>
where
    F: Fn(&Vec<u8>) -> bool,
{
    let (begin, end) = split_string_by_dash(input);
    let mut curr = begin;
    let mut valid_password_count = 0;
    while is_lexicographically_le(&curr, &end) {
        if is_valid_password(&curr) {
            valid_password_count += 1
        }
        lexicographically_increment_vec(&mut curr);
    }
    Some(valid_password_count)
}

pub fn part_one(input: &str) -> Option<u32> {
    compute(input, is_valid_part_one_password)
}

pub fn part_two(input: &str) -> Option<u32> {
    compute(input, is_valid_part_two_password)
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
        assert_eq!(part_one("123444-123444"), Some(1));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("111111-111111"), Some(0));
        assert_eq!(part_two("223450-223450"), Some(0));
        assert_eq!(part_two("123789-123789"), Some(0));
        assert_eq!(part_two("123455-123466"), Some(2));

        assert_eq!(part_two("112233-112233"), Some(1));
        assert_eq!(part_two("123444-123444"), Some(0));
        assert_eq!(part_two("111122-111122"), Some(1));
    }
}
