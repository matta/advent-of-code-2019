use std::{collections::HashMap, io};

// Parse the input string into a vector of tuples (String, String).  Each input
// line is parsed as two integral numbers separated by a ')' character.  The
// first number is the parent and the second the child.
fn parse(input: &str) -> Result<Vec<(String, String)>, io::Error> {
    let mut result = Vec::new();
    for line in input.trim().lines() {
        if let Some((parent, child)) = line.split_once(')') {
            result.push((parent.to_string(), child.to_string()));
        } else {
            // Handle the error case where line.split_once(')') returns None
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid input format",
            ));
        }
    }
    Ok(result)
}

fn part_one(input: &str) -> u32 {
    let orbits = parse(input).expect("invalid input");

    // direct_orbits is a hash map from primary planets to the secondary planets
    // that directly orbit them.
    let mut direct_orbits = HashMap::new();
    for (primary, secondary) in &orbits {
        direct_orbits
            .entry(primary.clone())
            .or_insert(Vec::new())
            .push(secondary.clone())
    }

    struct Frontier {
        orbit_count: u32,
        name: String,
    }

    // Declare a queue of Frontier objects and begin the breadth first traversal
    // with the "COM" planet.
    let mut queue = Vec::new();
    queue.push(Frontier {
        orbit_count: 0,
        name: "COM".to_string(),
    });

    // Count all orbits in the system.
    let mut total_orbit_count = 0;

    while let Some(primary) = queue.pop() {
        total_orbit_count += primary.orbit_count;
        if let Some(secondaries) = direct_orbits.get(&primary.name) {
            for secondary in secondaries {
                queue.push(Frontier {
                    orbit_count: primary.orbit_count + 1,
                    name: secondary.clone(),
                })
            }
        }
    }

    total_orbit_count
}

fn primaries_from(primaries: &HashMap<String, String>, secondary: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut curr = secondary;
    while let Some(next) = primaries.get(curr) {
        result.push(next.clone());
        curr = next;
    }
    result
}

fn part_two(input: &str) -> u32 {
    let orbits = parse(input).expect("invalid input");

    let mut primaries = HashMap::new();
    for (primary, secondary) in &orbits {
        primaries.insert(secondary.clone(), primary.clone());
    }

    let you_path = primaries_from(&primaries, "YOU");
    let san_path = primaries_from(&primaries, "SAN");

    let mut common_suffix = 0;
    for (a, b) in std::iter::zip(you_path.iter().rev(), san_path.iter().rev()) {
        if a != b {
            break;
        }
        common_suffix += 1;
    }

    let orbital_transfer = you_path.len() + san_path.len() - 2 * common_suffix;
    orbital_transfer as u32
}

fn main() {
    let input = include_str!("../inputs/06.txt").trim();
    let one = part_one(input);
    let two = part_two(input);
    assert_eq!(one, 333679);
    assert_eq!(two, 370);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
        assert_eq!(part_one(input), 42);
    }

    #[test]
    fn test_part_two() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
        assert_eq!(part_two(input), 4);
    }

    #[test]
    fn test_main() {
        main();
    }
}
