use std::collections::HashMap;

pub fn part1(input: &str) -> i64 {
    let mut memo: HashMap<(i64, i32), i64> = HashMap::new();
    parse(input)
        .into_iter()
        .map(|stone| count_stones(stone, 25, &mut memo))
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let mut memo: HashMap<(i64, i32), i64> = HashMap::new();
    parse(input)
        .into_iter()
        .map(|stone| count_stones(stone, 75, &mut memo))
        .sum()
}

fn parse(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .map(str::trim)
        .filter_map(|s| s.parse::<i64>().ok())
        .collect()
}

fn count_stones(stone: i64, blinks_left: i32, memo: &mut HashMap<(i64, i32), i64>) -> i64 {
    if blinks_left == 0 {
        return 1;
    }
    let key = (stone, blinks_left);
    if let Some(&result) = memo.get(&key) {
        return result;
    }
    let n = (stone.abs() as f64).log10().floor() as i64 + 1;
    let result = if stone != 0 {
        if n % 2 == 0 {
            let pow = 10_i64.pow((n / 2) as u32);
            count_stones(stone / pow, blinks_left - 1, memo)
                + count_stones(stone % pow, blinks_left - 1, memo)
        } else {
            count_stones(stone * 2024, blinks_left - 1, memo)
        }
    } else {
        count_stones(1, blinks_left - 1, memo)
    };
    memo.insert(key, result);
    result
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
        let input = load_input("input/day11/example.txt");

        let result = part1(&input);
        assert_eq!(result, 189541, "Failed on example case for part1");
    }

    #[test]
    fn test_my_input_case_part1() {
        let input = load_input("input/day11/input.txt");

        let result = part1(&input);
        assert_eq!(result, 184927, "Failed on my input case for part1");
    }

    #[test]
    fn test_example_case_part2() {
        let input = load_input("input/day11/example.txt");

        let result = part2(&input);
        assert_eq!(result, 226596360258785, "Failed on my input case for part1");
    }

    #[test]
    fn test_my_input_case_part2() {
        let input = load_input("input/day11/input.txt");

        let result = part2(&input);
        assert_eq!(result, 220357186726677, "Failed on my input case for part1");
    }
}
