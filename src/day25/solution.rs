fn get_height(column: &str) -> usize {
    column
        .chars()
        .filter(|&c| c == '#')
        .count()
        .saturating_sub(1)
}

fn parse_schematic(schematic: &str) -> Vec<usize> {
    let rows: Vec<&str> = schematic.lines().collect();
    let num_cols = rows.first().map_or(0, |row| row.len());
    let mut heights = Vec::with_capacity(num_cols);

    for col in 0..num_cols {
        let column: String = rows
            .iter()
            .map(|row| row.chars().nth(col).unwrap_or('.'))
            .collect();
        heights.push(get_height(&column));
    }

    heights
}

fn can_fit(lock: &[usize], key: &[usize]) -> bool {
    lock.iter().zip(key.iter()).all(|(&l, &k)| l + k <= 5)
}

pub fn part1(input: &str) -> usize {
    let schematics: Vec<&str> = input.trim().split("\n\n").collect();
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for schematic in schematics {
        let heights = parse_schematic(schematic);
        if schematic.starts_with("#####") {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    locks
        .iter()
        .flat_map(|lock| keys.iter().filter(move |key| can_fit(lock, key)))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn load_input(file_path: &str) -> String {
        fs::read_to_string(file_path).expect(&format!("Failed to read input file: {}", file_path))
    }

    #[test]
    fn test_example_case_part1() {
        let input = load_input("input/day25/example.txt");

        let result = part1(&input);
        assert_eq!(result, 3, "Failed on example case for part1");
    }

    #[test]
    fn test_my_input_case_part1() {
        let input = load_input("input/day25/input.txt");

        let result = part1(&input);
        assert_eq!(result, 2978, "Failed on my input case for part1");
    }
}
