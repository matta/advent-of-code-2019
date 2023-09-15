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

pub fn part_one(input: &str) -> Option<u32> {
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

    Some(total_orbit_count)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
        assert_eq!(part_one(input), Some(42));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
