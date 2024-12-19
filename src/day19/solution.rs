use std::collections::HashSet;

pub fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut sections = input.split("\n\n");

    let towels = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(", ")
                .map(|c| {
                    c.chars()
                        .map(|ch| "wubrg".find(ch).unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .flatten() // Flatten to make it a Vec<Vec<usize>>
        .collect();

    let patterns = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| "wubrg".find(ch).unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    (towels, patterns)
}

pub fn part1(input_map: &str) -> usize {
    let input = parse_input(input_map);

    // Initialize towels as a vector of hash sets
    let mut towels: Vec<HashSet<Vec<usize>>> = vec![];

    // Fill towels with the provided input
    for towel in &input.0 {
        while towels.len() <= towel.len() {
            towels.push(HashSet::new());
        }
        towels[towel.len()].insert(towel.clone());
    }

    let mut count = 0;

    // For each pattern, check if it's reachable
    for pattern in &input.1 {
        let mut reachable = vec![false; pattern.len() + 1];
        reachable[0] = true;

        // Try to find a reachable pattern by checking each previous pattern length
        for i in 1..=pattern.len() {
            for j in i.saturating_sub(towels.len() - 1)..i {
                if reachable[j] && towels[i - j].contains(&pattern[j..i]) {
                    reachable[i] = true;
                    break;
                }
            }
        }

        // If the final pattern is reachable, increment the count
        if reachable[pattern.len()] {
            count += 1;
        }
    }

    count
}

pub fn part2(input_map: &str) -> usize {
    let input = parse_input(input_map);
    let towels = build_towels(&input.0);

    let mut count = 0;
    for pattern in &input.1 {
        count += count_reachable_patterns(pattern, &towels);
    }

    count
}

/// Builds the towels data structure from the input.
fn build_towels(towels_input: &[Vec<usize>]) -> Vec<HashSet<Vec<usize>>> {
    let mut towels: Vec<HashSet<Vec<usize>>> = Vec::new();

    for towel in towels_input {
        while towels.len() <= towel.len() {
            towels.push(HashSet::new());
        }
        towels[towel.len()].insert(towel.clone());
    }

    towels
}

/// Counts the reachable patterns for a given pattern using the towels.
fn count_reachable_patterns(pattern: &[usize], towels: &[HashSet<Vec<usize>>]) -> usize {
    let mut reachable = vec![0; pattern.len() + 1];
    reachable[0] = 1;

    for i in 1..=pattern.len() {
        for j in i.saturating_sub(towels.len() - 1)..i {
            if reachable[j] != 0 && towels[i - j].contains(&pattern[j..i]) {
                reachable[i] += reachable[j];
            }
        }
    }

    reachable[pattern.len()]
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
        let input = load_input("input/day19/example.txt");

        let result = part1(&input);
        assert_eq!(result, 6, "Failed on example case for part1");
    }

    #[test]
    fn test_my_input_case_part1() {
        let input = load_input("input/day19/input.txt");

        let result = part1(&input);
        assert_eq!(result, 313, "Failed on my input case for part1");
    }

    #[test]
    fn test_my_input_case_part22() {
        let input = load_input("input/day19/example.txt");

        let result = part2(&input);
        assert_eq!(result, 16, "Failed on my input case for part1");
    }

    #[test]
    fn test_my_input_case_part2() {
        let input = load_input("input/day19/input.txt");

        let result = part2(&input);
        assert_eq!(result, 666491493769758, "Failed on my input case for part1");
    }
}
