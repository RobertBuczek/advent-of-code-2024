use crate::utils::strings::split_element_by_element_array_input_into_vector;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

fn find_in_grid(grid: &Vec<Vec<char>>, element: char) -> Option<(usize, usize)> {
    let n = grid.len();
    let m = grid[0].len();

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == element {
                return Some((i, j));
            }
        }
    }

    None
}

pub fn part1(input_map: &str) -> usize {
    let grid: Vec<Vec<char>> = split_element_by_element_array_input_into_vector(input_map);

    let (n, m) = (grid.len(), grid[0].len());

    // Find the start (S) and end (E) locations
    let cur_loc = find_in_grid(&grid, 'S').expect("Start location not found");
    let nd_loc = find_in_grid(&grid, 'E').expect("End location not found");

    // Initial state for Dijkstra's algorithm
    let cur_st = (cur_loc, (0, 1));
    let mut dijk_q: BinaryHeap<(i32, ((usize, usize), (i32, i32)))> = BinaryHeap::new();
    let mut d_map: HashMap<((usize, usize), (i32, i32)), i32> = HashMap::new();

    dijk_q.push((0, cur_st));
    d_map.insert(cur_st, 0);

    while let Some((cur_d, cur_st)) = dijk_q.pop() {
        let cur_d = -cur_d; // Convert back to positive distance

        // Skip if this is not the shortest path to the current state
        if let Some(&known_d) = d_map.get(&cur_st) {
            if known_d < cur_d {
                continue;
            }
        }

        // Check if we reached the end
        if cur_st.0 == nd_loc {
            return cur_d as usize;
        }

        let (cur_p, cur_dir) = cur_st;
        let new_p = (
            (cur_p.0 as i32 + cur_dir.0) as usize,
            (cur_p.1 as i32 + cur_dir.1) as usize,
        );

        // Add the new position if valid
        if new_p.0 < n && new_p.1 < m && grid[new_p.0][new_p.1] != '#' {
            let new_st = (new_p, cur_dir);
            let new_d = cur_d + 1;

            if !d_map.contains_key(&new_st) || d_map[&new_st] > new_d {
                d_map.insert(new_st, new_d);
                dijk_q.push((-new_d, new_st)); // Negative because BinaryHeap is max-heap
            }
        }

        // Add other directions (change direction)
        let mut other_dirs = vec![(1, 0), (-1, 0)];
        if other_dirs.contains(&cur_dir) {
            other_dirs = vec![(0, 1), (0, -1)];
        }

        for &other_dir in &other_dirs {
            let new_st = (cur_p, other_dir);
            let new_d = cur_d + 1000;

            if !d_map.contains_key(&new_st) || d_map[&new_st] > new_d {
                d_map.insert(new_st, new_d);
                dijk_q.push((-new_d, new_st));
            }
        }
    }

    // If we exit the loop without returning, no path was found
    0
}

pub fn part2(input_map: &str) -> usize {
    let grid: Vec<Vec<char>> = split_element_by_element_array_input_into_vector(input_map);

    let n = grid.len();
    let m = grid[0].len();

    let cur_loc = find_in_grid(&grid, 'S').expect("Start location not found");
    let nd_loc = find_in_grid(&grid, 'E').expect("End location not found");

    // Define the directions
    let directions: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    // Function to perform Dijkstra's algorithm
    fn solve(
        grid: &Vec<Vec<char>>,
        start: ((usize, usize), (i32, i32)),
        n: usize,
        m: usize,
        op_sign: bool,
    ) -> HashMap<((usize, usize), (i32, i32)), i32> {
        let mut dijk_q = BinaryHeap::new();
        let mut d_map = HashMap::new();

        dijk_q.push((Reverse(0), start));
        d_map.insert(start, 0);

        while let Some((Reverse(cur_d), cur_st)) = dijk_q.pop() {
            if let Some(&known_d) = d_map.get(&cur_st) {
                if known_d < cur_d {
                    continue;
                }
            }

            let (cur_p, cur_dir) = cur_st;
            let sgn = if op_sign { -1 } else { 1 };
            let new_p = (
                (cur_p.0 as i32 + sgn * cur_dir.0) as usize,
                (cur_p.1 as i32 + sgn * cur_dir.1) as usize,
            );

            if new_p.0 < n && new_p.1 < m && grid[new_p.0][new_p.1] != '#' {
                let new_st = (new_p, cur_dir);
                let new_d = cur_d + 1;

                if !d_map.contains_key(&new_st) || d_map[&new_st] > new_d {
                    d_map.insert(new_st, new_d);
                    dijk_q.push((Reverse(new_d), new_st));
                }
            }

            let mut other_dirs = vec![(1, 0), (-1, 0)];
            if other_dirs.contains(&cur_dir) {
                other_dirs = vec![(0, 1), (0, -1)];
            }

            for &other_dir in &other_dirs {
                let new_st = (cur_p, other_dir);
                let new_d = cur_d + 1000;

                if !d_map.contains_key(&new_st) || d_map[&new_st] > new_d {
                    d_map.insert(new_st, new_d);
                    dijk_q.push((Reverse(new_d), new_st));
                }
            }
        }

        d_map
    }

    // Initial solve for the starting point
    let init_solve = solve(&grid, (cur_loc, (0, 1)), n, m, false);
    let ans = directions
        .iter()
        .filter_map(|&dir| init_solve.get(&(nd_loc, dir)).cloned())
        .min()
        .unwrap_or(i32::MAX);

    let nd_solves: Vec<_> = directions
        .iter()
        .map(|&dir| solve(&grid, (nd_loc, dir), n, m, true))
        .collect();

    let orig_ans = ans;
    let mut ans_set: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..n {
        for j in 0..m {
            for &dir in &directions {
                let cur_st = ((i, j), dir);
                let mut good = false;

                for nd_solve in &nd_solves {
                    if let (Some(&init_dist), Some(&nd_dist)) =
                        (init_solve.get(&cur_st), nd_solve.get(&cur_st))
                    {
                        let try_ans = init_dist + nd_dist;

                        if try_ans == orig_ans {
                            good = true;
                            break;
                        }
                    }
                }

                if good {
                    ans_set.insert(cur_st.0);
                }
            }
        }
    }

    ans_set.len()
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
        let input = load_input("input/day16/example.txt");

        let result = part1(&input);
        assert_eq!(result, 7036, "Failed on example case for part1");
    }

    #[test]
    fn test_my_input_case_part1() {
        let input = load_input("input/day16/input.txt");

        let result = part1(&input);
        assert_eq!(result, 85396, "Failed on my input case for part1");
    }

    #[test]
    fn test_example_case_part2() {
        let input = load_input("input/day16/example.txt");

        let result = part2(&input);
        assert_eq!(result, 45, "Failed on my input case for part1");
    }

    #[test]
    fn test_my_input_case_part2() {
        let input = load_input("input/day16/input.txt");

        let result = part2(&input);
        assert_eq!(result, 428, "Failed on my input case for part1");
    }
}
