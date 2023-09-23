fn parse(input: &str) -> Vec<u8> {
    // Parse each input character as a single digit integer, returning a vector of u8.
    input.trim().bytes().map(|c| c - b'0').collect()
}

fn count_occurrences<T: Eq + Copy>(sequence: &[T], target: T) -> usize {
    sequence
        .iter()
        .fold(0, |acc, &x| if x == target { acc + 1 } else { acc })
}

pub fn part_one(input: &str) -> Option<u32> {
    let image = parse(input);
    let width = 25;
    let height = 6;
    let layer_size = width * height;

    // Count the number of 0s in each layer.
    let mut zeroes = Vec::new();
    for layer in image.chunks(layer_size) {
        assert!(layer.len() == layer_size);
        zeroes.push(count_occurrences(layer, 0));
    }

    // Find the layer with the fewest number of zeroes.
    let mut lowest_count = usize::MAX;
    let mut layer: usize = 0;
    zeroes.iter().enumerate().for_each(|(x, &count)| {
        if count <= lowest_count {
            lowest_count = count;
            layer = x;
        }
    });

    let layer_data = &image[layer * layer_size..(layer + 1) * layer_size];
    assert!(layer_data.len() == layer_size);
    Some((count_occurrences(layer_data, 1) * count_occurrences(layer_data, 2)) as u32)
}

fn render_image(input: &str, width: usize, height: usize) -> Vec<u8> {
    let image = parse(input);
    let layer_size = width * height;

    let mut rendered_image = vec![2; layer_size];
    for i in 0..layer_size {
        for layer in image.chunks(layer_size) {
            if rendered_image[i] == 2 {
                rendered_image[i] = layer[i];
            }
        }
    }

    rendered_image
}

pub fn part_two(input: &str) -> Option<u32> {
    let width = 25;
    let height = 6;
    let image = render_image(input, width, height);
    for row in image.chunks(width) {
        for pixel in row {
            // Print each pixel in decimal format.
            print!("{}", if *pixel == 0 { ' ' } else { '#' });
        }
        println!();
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "0123479";
        assert_eq!(parse(input), vec![0, 1, 2, 3, 4, 7, 9]);
    }

    #[test]
    fn test_render_image() {
        let input = "0222112222120000";
        let image = render_image(input, 2, 2);
        assert_eq!(image, vec![0, 1, 1, 0]);
    }
}
