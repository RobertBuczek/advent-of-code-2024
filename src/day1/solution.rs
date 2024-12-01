use crate::utils;
use std::collections::HashMap;

pub fn solve_part1(input: &str) -> i32 {
    let (mut a_vec, mut b_vec): (Vec<i32>, Vec<i32>) =
        utils::strings::split_input_into_vector(input)
            .iter()
            .cloned()
            .unzip();

    a_vec.sort();
    b_vec.sort();

    a_vec
        .iter()
        .zip(b_vec.iter())
        .map(|(a, b)| a.abs_diff(*b) as i32)
        .sum()
}

pub fn solve_part2(input: &str) -> i32 {
    let (a_vec, b_vec): (Vec<_>, Vec<_>) = utils::strings::split_input_into_vector(input)
        .iter()
        .cloned()
        .unzip();
    let group_and_count: HashMap<i32, i32> = utils::maps::group_and_count(b_vec);
    let mut sum: i32 = 0;

    for &item in a_vec.iter() {
        if let Some(&count) = group_and_count.get(&item) {
            sum += count * &item;
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test_my_case_case() {
        let result = solve_part1(
            &&fs::read_to_string("input/day1/input.txt").expect("Failed to read input file"),
        );
        assert_eq!(result, 2815556);
    }

    #[test]
    fn test_my_case_part_2_case() {
        let result = solve_part2(
            &fs::read_to_string("input/day1/input.txt").expect("Failed to read input file"),
        );
        assert_eq!(result, 23927637);
    }

    #[test]
    fn test_basic_case() {
        let input = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3";
        let result = solve_part1(&input);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_example_case() {
        let input =
            fs::read_to_string("input/day1/example.txt").expect("Failed to read input file");

        let result = solve_part1(&input);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let result = solve_part1(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_single_pair() {
        let input = "1 2";
        let result = solve_part1(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_sorted_input() {
        let input = "1 2\n2 3\n3 4\n4 5";
        let result = solve_part1(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_with_equal_numbers() {
        let input = "1 1\n2 2\n3 3\n4 4";
        let result = solve_part1(input);
        assert_eq!(result, 0);
    }
}
