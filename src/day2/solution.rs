use crate::utils;

pub fn part1(input: &str) -> usize {
    utils::strings::split_array_input_into_vector(input)
        .into_iter()
        .filter(|report| is_safe_report(report))
        .count()
}

pub fn part2(input: &str) -> usize {
    utils::strings::split_array_input_into_vector(input)
        .into_iter()
        .filter(|report| is_safe_with_dampener(report))
        .count()
}

fn is_safe_report(report: &Vec<i32>) -> bool {
    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];
        if diff.abs() < 1
            || diff.abs() > 3
            || (diff > 0 && i > 1 && report[i - 1] < report[i - 2])
            || (diff < 0 && i > 1 && report[i - 1] > report[i - 2])
        {
            return false;
        }
    }
    true
}

fn is_safe_with_dampener(report: &Vec<i32>) -> bool {
    if is_safe_report(report) {
        return true;
    }

    (0..report.len()).any(|i| {
        let modified_report = utils::vecs::remove_at_index(&report, i);
        is_safe_report(&modified_report)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn test_example_case() {
        let input =
            fs::read_to_string("input/day2/example.txt").expect("Failed to read input file");

        let result = part1(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_my_input_example_case() {
        let input = fs::read_to_string("input/day2/input.txt").expect("Failed to read input file");

        let result = part1(&input);
        assert_eq!(result, 490);
    }

    #[test]
    fn test_example_case_part_2() {
        let input =
            fs::read_to_string("input/day2/example.txt").expect("Failed to read input file");

        let result = part2(&input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_my_input_case_part_2() {
        let input = fs::read_to_string("input/day2/input.txt").expect("Failed to read input file");

        let result = part2(&input);
        assert_eq!(result, 536);
    }
}
