use regex::{Captures, Regex};
use std::sync::atomic::{AtomicBool, Ordering};

pub fn part1(input: &str) -> i32 {
    let re: Regex = Regex::new(r"mul\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\)").unwrap();

    re.captures_iter(input)
        .map(|cap: Captures| {
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();
            x * y
        })
        .sum()
}

pub fn part2(input: &str) -> i32 {
    let re: Regex = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))").unwrap();
    let atomic = AtomicBool::new(true);

    re.captures_iter(input)
        .filter_map(|captures| match &captures[0] {
            "do()" => {
                atomic.store(true, Ordering::Relaxed);
                None
            }
            "don't()" => {
                atomic.store(false, Ordering::Relaxed);
                None
            }
            mul if mul.starts_with("mul(") && atomic.load(Ordering::Relaxed) => {
                if let (Ok(a), Ok(b)) = (captures[2].parse::<i32>(), captures[3].parse::<i32>()) {
                    Some(a * b)
                } else {
                    None
                }
            }
            _ => None,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example_case() {
        let input =
            fs::read_to_string("input/day3/example.txt").expect("Failed to read input file");
        let result = part1(&input);
        assert_eq!(result, 161);
    }

    #[test]
    fn test_my_input_example_case() {
        let input = fs::read_to_string("input/day3/input.txt").expect("Failed to read input file");
        let result = part1(&input);
        assert_eq!(result, 170068701);
    }

    #[test]
    fn test_example_case_part_2() {
        let input =
            fs::read_to_string("input/day3/example.txt").expect("Failed to read input file");
        let result = part2(&input);
        assert_eq!(result, 48);
    }

    #[test]
    fn test_my_input_case_part_2() {
        let input = fs::read_to_string("input/day3/input.txt").expect("Failed to read input file");
        let result = part2(&input);
        assert_eq!(result, 78683433);
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        assert_eq!(part1(input), 0);
        assert_eq!(part2(input), 0);
    }
}
