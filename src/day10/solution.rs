use crate::utils::strings::split_element_by_element_array_input_into_vector;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn part1(input: &str) -> usize {
    let grid = split_element_by_element_array_input_into_vector(input);
    find_trailheads(&grid)
        .iter()
        .map(|&trailhead| bfs_reachable_nines(&grid, trailhead))
        .sum()
}

fn find_trailheads(grid: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &height)| height == 0)
                .map(move |(c, _)| (r, c))
        })
        .collect()
}

fn is_valid_move(grid: &[Vec<u8>], current: (usize, usize), next: (isize, isize)) -> bool {
    let (rows, cols) = (grid.len() as isize, grid[0].len() as isize);
    let (cr, cc) = (current.0 as isize, current.1 as isize);
    let (nr, nc) = next;

    if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
        let current_height = grid[cr as usize][cc as usize];
        let next_height = grid[nr as usize][nc as usize];
        return next_height == current_height + 1;
    }
    false
}

fn bfs_reachable_nines(grid: &[Vec<u8>], start: (usize, usize)) -> usize {
    let mut queue = VecDeque::from([start]);
    let mut visited = HashSet::from([start]);
    let mut reachable_nines = HashSet::new();

    while let Some((cr, cc)) = queue.pop_front() {
        let current_height = grid[cr][cc];

        if current_height == 9 {
            reachable_nines.insert((cr, cc));
            continue;
        }

        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next_pos = ((cr as isize + dr) as usize, (cc as isize + dc) as usize);
            let (nr, nc) = (cr as isize + dr, cc as isize + dc);

            if !visited.contains(&next_pos) && is_valid_move(grid, (cr, cc), (nr, nc)) {
                queue.push_back(next_pos);
                visited.insert(next_pos);
            }
        }
    }

    reachable_nines.len()
}

pub fn part2(input: &String) -> usize {
    let grid = split_element_by_element_array_input_into_vector(input);

    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();

    find_trailheads(&grid)
        .iter()
        .map(|&(i, j)| calculate_rating(i, j, &grid, &mut memo))
        .sum()
}

fn calculate_rating(
    i: usize,
    j: usize,
    grid: &[Vec<u8>],
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(&cached) = memo.get(&(i, j)) {
        return cached;
    }

    if grid[i][j] == 9 {
        memo.insert((i, j), 1);
        return 1;
    }

    let mut total_rating = 0;
    for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let next_i = i as isize + di;
        let next_j = j as isize + dj;

        if is_within_bounds(next_i, next_j, grid.len()) {
            let next_i = next_i as usize;
            let next_j = next_j as usize;

            if grid[next_i][next_j] == grid[i][j] + 1 {
                total_rating += calculate_rating(next_i, next_j, grid, memo);
            }
        }
    }

    memo.insert((i, j), total_rating);
    total_rating
}

fn is_within_bounds(i: isize, j: isize, n: usize) -> bool {
    i >= 0 && i < n as isize && j >= 0 && j < n as isize
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
        let input = load_input("input/day10/example.txt");

        let result = part1(&input);
        assert_eq!(result, 36, "Failed on example case for part1");
    }

    #[test]
    fn test_my_input_case_part1() {
        let input = load_input("input/day10/input.txt");

        let result = part1(&input);
        assert_eq!(result, 611, "Failed on my input case for part1");
    }

    #[test]
    fn test_example_case_part2() {
        let input = load_input("input/day10/example.txt");

        let result = part2(&input);
        assert_eq!(result, 81, "Failed on my input case for part1");
    }

    #[test]
    fn test_my_input_case_part2() {
        let input = load_input("input/day10/input.txt");

        let result = part2(&input);
        assert_eq!(result, 1380, "Failed on my input case for part1");
    }
}
