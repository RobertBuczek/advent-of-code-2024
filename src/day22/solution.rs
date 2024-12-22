use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> i64 {
    const ITERATIONS: usize = 2000;
    let mut ans = 0;

    for mut value in extract(input) {
        for _ in 0..ITERATIONS {
            value = next_num(value);
        }
        ans += value;
    }

    ans
}

pub fn part2(input: &str) -> i64 {
    const ITERATIONS: usize = 2000;
    let mut map = HashMap::new();

    for v in extract(input) {
        let mut visited = v;
        let mut seen = HashSet::new();

        let mut arr = [0, 0, 0, 0];
        let mut prev_price = 0;

        for iteration in 0..ITERATIONS {
            visited = next_num(visited);
            let price = visited % 10;
            arr = [price - prev_price, arr[0], arr[1], arr[2]];

            if seen.insert(arr) && iteration >= 4 {
                *map.entry(arr).or_default() += price;
            }

            prev_price = price;
        }
    }

    *map.values().max().expect("No records")
}

fn extract(input: &str) -> Vec<i64> {
    input
        .lines()
        .filter_map(|line| line.trim().parse::<i64>().ok())
        .collect()
}

fn next_num(mut x: i64) -> i64 {
    const MASK: i64 = 0x00FFFFFF;

    x ^= x << 6;
    x &= MASK;
    x ^= x >> 5;
    x &= MASK;
    x ^= x << 11;
    x &= MASK;
    x
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
        let input = load_input("input/day22/example.txt");

        let result = part1(&input);
        assert_eq!(result, 94558292, "Failed on example case for part1");
    }

    #[test]
    fn test_my_input_case_part1() {
        let input = load_input("input/day22/input.txt");

        let result = part1(&input);
        assert_eq!(result, 19927218456, "Failed on my input case for part1");
    }

    #[test]
    fn test_my_input_case_part22() {
        let input = load_input("input/day22/example.txt");

        let result = part2(&input);
        assert_eq!(result, 90, "Failed on my input case for part1");
    }

    #[test]
    fn test_my_input_case_part2() {
        let input = load_input("input/day22/input.txt");

        let result = part2(&input);
        assert_eq!(result, 2189, "Failed on my input case for part1");
    }
}
