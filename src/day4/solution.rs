use crate::utils;
use std::collections::HashMap;

fn part1(input: &str) -> i32 {
    let vec: Vec<Vec<char>> =
        utils::strings::split_element_by_element_array_input_into_vector(&input);

    let vec1: Vec<String> = get_all_permutations_with_window(&vec, 4);
    let map: HashMap<String, i32> = utils::maps::group_and_count_by_type(vec1);
    let xmas_count: i32 = *map.get("XMAS").unwrap_or(&0);
    let samx_count: i32 = *map.get("SAMX").unwrap_or(&0);

    xmas_count + samx_count
}

fn part2(input: &str) -> i32 {
    let grid: Vec<Vec<char>> =
        utils::strings::split_element_by_element_array_input_into_vector(&input);

    let mut count = 0;
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    for row in 1..(rows - 1) {
        for col in 1..(cols - 1) {
            if is_xmas_pattern(&grid, row, col) {
                count += 1;
            }
        }
    }

    count
}

fn is_xmas_pattern(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    if row == 0 || col == 0 || row + 1 >= grid.len() || col + 1 >= grid[0].len() {
        return false;
    }

    if grid[row][col] != 'A' {
        return false;
    }

    let check_pattern = |left_top, left_bottom, right_top, right_bottom| {
        [
            left_top == 'M' && left_bottom == 'M' && right_top == 'S' && right_bottom == 'S',
            left_top == 'S' && left_bottom == 'M' && right_top == 'S' && right_bottom == 'M',
            left_top == 'M' && left_bottom == 'S' && right_top == 'M' && right_bottom == 'S',
            left_top == 'S' && left_bottom == 'S' && right_top == 'M' && right_bottom == 'M',
        ]
        .iter()
        .any(|&x| x)
    };

    let left_top = grid[row - 1][col - 1];
    let left_bottom = grid[row + 1][col - 1];
    let right_top = grid[row - 1][col + 1];
    let right_bottom = grid[row + 1][col + 1];

    check_pattern(left_top, left_bottom, right_top, right_bottom)
}

pub fn get_all_permutations_with_window(grid: &Vec<Vec<char>>, window_size: usize) -> Vec<String> {
    let mut permutations = Vec::new();
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    for row in grid.iter() {
        for start in 0..=(row.len().saturating_sub(window_size)) {
            permutations.push(row[start..start + window_size].iter().collect::<String>());
        }
    }

    // Vertical permutations
    for col in 0..cols {
        for start in 0..=(rows.saturating_sub(window_size)) {
            let vertical: String = (start..start + window_size)
                .map(|row| grid[row][col])
                .collect();
            permutations.push(vertical);
        }
    }

    // Diagonal (Top-left to Bottom-right)
    for start_row in 0..=(rows.saturating_sub(window_size)) {
        for start_col in 0..=(cols.saturating_sub(window_size)) {
            let diagonal: String = (0..window_size)
                .map(|i| grid[start_row + i][start_col + i])
                .collect();
            permutations.push(diagonal);
        }
    }

    // Diagonal (Top-right to Bottom-left)
    for start_row in 0..=(rows.saturating_sub(window_size)) {
        for start_col in (window_size - 1)..cols {
            let diagonal: String = (0..window_size)
                .map(|i| grid[start_row + i][start_col - i])
                .collect();
            permutations.push(diagonal);
        }
    }

    permutations
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn test_example_case() {
        let input =
            fs::read_to_string("input/day4/example.txt").expect("Failed to read input file");

        let result = part1(&input);
        assert_eq!(result, 18); // Example expected result, adjust accordingly
    }

    #[test]
    fn test_my_input_example_case() {
        let input = fs::read_to_string("input/day4/input.txt").expect("Failed to read input file");

        let result = part1(&input);
        assert_eq!(result, 2569); // Example expected result, adjust accordingly
    }

    #[test]
    fn test_example_case_part_2() {
        let input =
            fs::read_to_string("input/day4/example.txt").expect("Failed to read input file");

        let result = part2(&input);
        assert_eq!(result, 9); // Example expected result, adjust accordingly
    }

    #[test]
    fn test_my_input_case_part_2() {
        let input = fs::read_to_string("input/day4/input.txt").expect("Failed to read input file");

        let result = part2(&input);
        assert_eq!(result, 1998);
    }
}
