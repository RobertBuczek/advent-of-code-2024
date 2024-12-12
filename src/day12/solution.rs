use crate::utils::dsu::DSU;
use crate::utils::strings::split_element_by_element_array_input_into_vector;
use std::collections::{HashMap, HashSet, VecDeque};

fn flood_fill(
    grid: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    visited: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    let rows = grid.len();
    let cols = grid[0].len();
    let plant_type = grid[x][y];
    let mut stack = VecDeque::new();
    let mut area = 0;
    let mut perimeter = 0;

    stack.push_back((x, y));

    while let Some((cx, cy)) = stack.pop_front() {
        if visited.contains(&(cx, cy)) {
            continue;
        }

        visited.insert((cx, cy));
        area += 1;

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = cx as isize + dx;
            let ny = cy as isize + dy;

            if nx >= 0 && nx < rows as isize && ny >= 0 && ny < cols as isize {
                let nx = nx as usize;
                let ny = ny as usize;
                if grid[nx][ny] == plant_type && !visited.contains(&(nx, ny)) {
                    stack.push_back((nx, ny));
                } else if grid[nx][ny] != plant_type {
                    perimeter += 1;
                }
            } else {
                perimeter += 1; // Out of bounds contributes to the perimeter
            }
        }
    }

    (area, perimeter)
}

pub fn part1(input_map: &str) -> usize {
    let grid = split_element_by_element_array_input_into_vector(input_map);
    let mut visited = HashSet::new();
    let mut total_cost = 0;

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if !visited.contains(&(x, y)) {
                let (area, perimeter) = flood_fill(&grid, x, y, &mut visited);
                total_cost += area * perimeter;
            }
        }
    }

    total_cost
}

pub fn part2(input_map: &str) -> i32 {
    let grid: Vec<Vec<char>> = split_element_by_element_array_input_into_vector(input_map);
    let n = grid.len();
    let mut uf = DSU::new(n * n);

    let to_coord = |a: usize, b: usize| a * n + b;

    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for i in 0..n {
        for j in 0..n {
            for (dx, dy) in dirs.iter() {
                let newx = i as isize + dx;
                let newy = j as isize + dy;
                if newx >= 0
                    && (newx as usize) < n
                    && newy >= 0
                    && (newy as usize) < n
                    && grid[i][j] == grid[newx as usize][newy as usize]
                {
                    uf.merge(to_coord(i, j), to_coord(newx as usize, newy as usize));
                }
            }
        }
    }

    let comps = uf.groups();
    let mut comp_map = HashMap::new();

    for x in &comps {
        for &y in x {
            comp_map.insert(y, x[0]);
        }
    }

    let mut bnds = HashMap::new();
    let mut extras = HashMap::new();

    for i in 0..=n {
        for j in 0..=n {
            let mut curcomps: HashMap<usize, HashSet<(isize, isize)>> = HashMap::new();
            let mut bad = false;

            for &(dx, dy) in &[(0, 0), (-1, -1), (-1, 0), (0, -1)] {
                let newx = i as isize + dx;
                let newy = j as isize + dy;

                if newx >= 0 && (newx as usize) < n && newy >= 0 && (newy as usize) < n {
                    let coord = to_coord(newx as usize, newy as usize);
                    curcomps
                        .entry(*comp_map.get(&coord).unwrap())
                        .or_insert_with(HashSet::new)
                        .insert((dx, dy));
                } else {
                    bad = true;
                }
            }

            if curcomps.len() != 1 || bad {
                for (&c, s) in &curcomps {
                    if s == &[(0, 0), (-1, -1)].iter().copied().collect()
                        || s == &[(0, -1), (-1, 0)].iter().copied().collect()
                    {
                        *bnds.entry(c).or_insert(0) += 2;
                    } else {
                        *bnds.entry(c).or_insert(0) += 1;
                        if s.len() == 2 {
                            *extras.entry(c).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for (&c, &b) in &bnds {
        let truth = comps.iter().find(|x| x.contains(&c)).unwrap().len();
        ans += (b - extras.get(&c).unwrap_or(&0)) as i32 * truth as i32;
    }

    ans
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
        let input = load_input("input/day12/example.txt");

        let result = part1(&input);
        assert_eq!(result, 1930, "Failed on example case for part1");
    }

    #[test]
    fn test_my_input_case_part1() {
        let input = load_input("input/day12/input.txt");

        let result = part1(&input);
        assert_eq!(result, 1573474, "Failed on my input case for part1");
    }

    #[test]
    fn test_example_case_part2() {
        let input = load_input("input/day12/example.txt");

        let result = part2(&input);
        assert_eq!(result, 1206, "Failed on my input case for part1");
    }

    #[test]
    fn test_my_input_case_part2() {
        let input = load_input("input/day12/input.txt");

        let result = part2(&input);
        assert_eq!(result, 966476, "Failed on my input case for part1");
    }
}
