use crate::utils::strings::split_element_by_element_array_input_into_vector;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = split_element_by_element_array_input_into_vector(input);
    let grid_len = grid.len();

    let find_antinodes = |a: (isize, isize), b: (isize, isize)| -> Vec<(isize, isize)> {
        let (ax, ay) = a;
        let (bx, by) = b;

        [
            (ax - (bx - ax), ay - (by - ay)),
            (bx + (bx - ax), by + (by - ay)),
        ]
        .into_iter()
        .filter(|&(x, y)| in_bounds(x, y, grid_len))
        .collect()
    };

    let mut result = HashSet::new();
    for positions in digest_chars(&grid).values() {
        for (a, b) in positions.iter().tuple_combinations() {
            result.extend(find_antinodes(*a, *b));
        }
    }

    result.len()
}

fn digest_chars(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<(isize, isize)>> {
    let mut char_positions: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch != '.' {
                char_positions
                    .entry(ch)
                    .or_default()
                    .push((i as isize, j as isize));
            }
        }
    }

    char_positions
}

fn in_bounds(x: isize, y: isize, n: usize) -> bool {
    x >= 0 && y >= 0 && x < n as isize && y < n as isize
}

pub fn part2(input: &String) -> usize {
    let grid: Vec<Vec<char>> = split_element_by_element_array_input_into_vector(input);
    let n = grid.len();

    let get_antinodes = |a: (isize, isize), b: (isize, isize)| -> Vec<(isize, isize)> {
        let (ax, ay) = a;
        let (bx, by) = b;
        let (dx, dy) = (bx - ax, by - ay);

        let mut antinodes = Vec::new();

        let mut i = 0;
        while let Some((nx, ny)) = ax.checked_sub(dx * i).zip(ay.checked_sub(dy * i)) {
            if in_bounds(nx, ny, n) {
                antinodes.push((nx, ny));
            } else {
                break;
            }
            i += 1;
        }

        i = 0;
        while let Some((nx, ny)) = bx.checked_add(dx * i).zip(by.checked_add(dy * i)) {
            if in_bounds(nx, ny, n) {
                antinodes.push((nx, ny));
            } else {
                break;
            }
            i += 1;
        }

        antinodes
    };

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for (_freq, locs) in &digest_chars(&grid) {
        for (a, b) in locs.iter().tuple_combinations() {
            for antinode in get_antinodes(*a, *b) {
                antinodes.insert(antinode);
            }
        }
    }

    antinodes.len()
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
        let input = load_input("input/day8/example.txt");

        let result = part1(&input);
        assert_eq!(result, 14, "Failed on example case for part1");
    }

    #[test]
    fn test_my_input_case_part1() {
        let input = load_input("input/day8/input.txt");

        let result = part1(&input);
        assert_eq!(result, 390, "Failed on my input case for part1");
    }

    #[test]
    fn test_my_input_case_part2() {
        let input = load_input("input/day8/input.txt");

        let result = part2(&input);
        assert_eq!(result, 1246, "Failed on my input case for part2");
    }

    #[test]
    fn test_part1_no_antinodes() {
        let input = "....\n....\n....\n....\n".to_string();
        let result = part1(&input);
        assert_eq!(result, 0); // No characters to form antinodes
    }

    #[test]
    fn test_part2_no_antinodes() {
        let input = "....\n....\n....\n....\n".to_string();
        let result = part2(&input);
        assert_eq!(result, 0); // No characters to form antinodes
    }

    #[test]
    fn test_part1_empty_grid() {
        let input = String::from("");
        let result = part1(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part1_no_non_dot() {
        let input = String::from("...\n...\n...");
        let result = part1(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part1_single_character() {
        let input = String::from(".A.\n...\n...");
        let result = part1(&input);
        assert_eq!(result, 0); // Only one 'A', no pairs to generate antinodes
    }

    #[test]
    fn test_part1_two_characters_no_antinode() {
        let input = String::from("A..\n...\n..A");
        let result = part1(&input);
        assert_eq!(result, 0); // Antinodes are out of bounds
    }

    #[test]
    fn test_part1_two_characters_with_antinode() {
        let input = String::from("..A..\n.....\nA....");
        let result = part1(&input);
        // Positions: (0,2) and (2,0)
        // Antinodes: (0 - (2-0), 2 - (0-2)) = (-2,4) out of bounds
        // (2 + (2-0), 0 + (0-2)) = (4,-2) out of bounds
        // Thus, expected=0
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part1_two_characters_with_in_bounds_antinode() {
        let input = String::from("A..A\n....\n....\nA..A");
        let result = part1(&input);
        // Positions: (0,0), (0,3), (3,0), (3,3)
        // Pairs:
        // (0,0)-(0,3): cx=0 - (0-0)=0, cy=0 - (3-0)= -3 (out)
        // dx=0 + (0-0)=0, dy=3 + (3-0)=6 (out)
        // (0,0)-(3,0): cx=0 - (3-0)= -3, cy=0 - (0-0)=0 (out)
        // dx=3 + (3-0)=6, dy=0 + (0-0)=0 (out)
        // (0,0)-(3,3): cx=0 - (3-0)= -3, cy=0 - (3-0)= -3 (out)
        // dx=3 + (3-0)=6, dy=3 + (3-0)=6 (out)
        // Similarly for other pairs
        // Thus, expected=0
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_empty_grid() {
        let input = String::from("");
        let result = part2(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_no_non_dot() {
        let input = String::from("...\n...\n...");
        let result = part2(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_single_character() {
        let input = String::from(".A.\n...\n...");
        let result = part2(&input);
        assert_eq!(result, 0); // Only one 'A', no pairs to generate antinodes
    }

    #[test]
    fn test_part1_different_characters() {
        let input = String::from("A.B\n.C.\nB.A");
        let result = part1(&input);
        // Characters: 'A', 'B', 'C'
        // 'A': (0,0), (2,2)
        // 'B': (0,2), (2,0)
        // 'C': (1,1)
        // For 'A' pair: antinodes (-2,-2), (4,4) out of bounds
        // For 'B' pair: antinodes (-2,4), (4,-2) out of bounds
        // 'C' has only one location
        // Thus, expected=0
        assert_eq!(result, 0);
    }
}
