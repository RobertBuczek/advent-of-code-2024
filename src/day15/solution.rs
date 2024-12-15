use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

pub fn part1(input_map: &str) -> usize {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut moves = String::new();
    let mut st = false;

    // Process the input map
    for line in input_map.lines() {
        if line.trim().is_empty() {
            st = true;
            continue;
        }
        if st {
            moves.push_str(line.trim());
        } else {
            grid.push(line.trim().chars().collect());
        }
    }

    let n = grid.len();
    let m = grid[0].len();

    // Find the initial location of "@"
    let mut cur_loc = None;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == '@' {
                cur_loc = Some((i, j));
                break;
            }
        }
    }

    // Define the direction mappings with i32 instead of usize
    let mut symtodir = HashMap::new();
    symtodir.insert('>', (0, 1)); // Right (no negative values)
    symtodir.insert('<', (0, -1)); // Left (negative x direction)
    symtodir.insert('^', (-1, 0)); // Up (negative y direction)
    symtodir.insert('v', (1, 0)); // Down (positive y direction)

    // Move processing
    for move_sym in moves.chars() {
        if let Some(dir) = symtodir.get(&move_sym) {
            let newloc = (
                cur_loc.unwrap().0 as i32 + dir.0,
                cur_loc.unwrap().1 as i32 + dir.1,
            );
            let mut finalloc = newloc;

            // Ensure we stay within bounds using wrapping (modular arithmetic)
            while grid[finalloc.0 as usize][finalloc.1 as usize] != '.'
                && grid[finalloc.0 as usize][finalloc.1 as usize] != '#'
            {
                finalloc = (finalloc.0 + dir.0, finalloc.1 + dir.1);
            }

            if grid[finalloc.0 as usize][finalloc.1 as usize] != '#' {
                let (cur_x, cur_y) = cur_loc.unwrap();
                let (new_x, new_y) = newloc;

                assert!(grid[new_x as usize][new_y as usize] != '#'); // Sanity check
                grid[finalloc.0 as usize][finalloc.1 as usize] =
                    grid[new_x as usize][new_y as usize];
                grid[new_x as usize][new_y as usize] = '@';
                grid[cur_x as usize][cur_y as usize] = '.';
                cur_loc = Some((new_x as usize, new_y as usize)); // Update the current location
            }
        }
    }

    // Calculate the result based on the 'O' positions
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 'O' {
                ans += 100 * i + j;
            }
        }
    }

    // Return the result
    ans
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    i: usize,
    j: usize,
}

#[derive(Debug)]
struct State {
    player: Position,
    boxes: Vec<Position>,
    walls: HashSet<Position>,
    grid_size: usize,
}

impl State {
    fn new(grid: &Vec<Vec<char>>) -> Self {
        let n = grid.len();
        let mut player = Position { i: 0, j: 0 };
        let mut boxes = Vec::new();
        let mut walls = HashSet::new();

        for i in 0..n {
            for j in 0..n {
                match grid[i][j] {
                    '@' => player = Position { i, j: j * 2 },
                    'O' => boxes.push(Position { i, j: j * 2 }),
                    '#' => {
                        walls.insert(Position { i, j: j * 2 });
                        walls.insert(Position { i, j: j * 2 + 1 });
                    }
                    _ => {}
                }
            }
        }

        Self {
            player,
            boxes,
            walls,
            grid_size: n,
        }
    }

    fn in_grid(&self, i: i32, j: i32) -> bool {
        i >= 0 && i < self.grid_size as i32 && j >= 0 && j < (2 * self.grid_size) as i32
    }

    fn move_player(&mut self, dir: (i32, i32)) {
        let new_i = self.player.i as i32 + dir.0;
        let new_j = self.player.j as i32 + dir.1;

        if !self.in_grid(new_i, new_j) {
            return;
        }

        let new_pos = Position {
            i: new_i as usize,
            j: new_j as usize,
        };

        if self.walls.contains(&new_pos) {
            return;
        }

        let mut stack = VecDeque::new();
        let mut seen = HashSet::new();

        for &box_pos in &self.boxes {
            let position = Position {
                i: new_i as usize,
                j: new_j as usize - 1,
            };
            if box_pos == new_pos || box_pos == position {
                stack.push_back(box_pos);
            }
        }

        let mut can_move = true;
        while let Some(top) = stack.pop_front() {
            let next_i = top.i as i32 + dir.0;
            let next_j = top.j as i32 + dir.1;

            if !self.in_grid(next_i, next_j)
                || self.walls.contains(&Position {
                    i: next_i as usize,
                    j: next_j as usize,
                })
                || self.walls.contains(&Position {
                    i: next_i as usize,
                    j: next_j as usize + 1,
                })
            {
                can_move = false;
                break;
            }

            if seen.contains(&top) {
                continue;
            }

            seen.insert(top);

            for &box_pos in &self.boxes {
                let position = Position {
                    i: new_i as usize,
                    j: new_j as usize,
                };
                let position_2 = Position {
                    i: new_i as usize,
                    j: new_j as usize - 1,
                };
                let position_3 = Position {
                    i: new_i as usize,
                    j: new_j as usize + 1,
                };
                if box_pos == position || box_pos == position_2 || box_pos == position_3 {
                    stack.push_back(box_pos);
                }
            }
        }

        if can_move {
            for box_pos in &mut self.boxes {
                if seen.contains(box_pos) {
                    box_pos.i = (box_pos.i as i32 + dir.0) as usize;
                    box_pos.j = (box_pos.j as i32 + dir.1) as usize;
                }
            }
            self.player.i = new_i as usize;
            self.player.j = new_j as usize;
        }
    }

    fn calculate_score(&self) -> i32 {
        self.boxes
            .iter()
            .map(|pos| (pos.i as i32) * 100 + (pos.j as i32))
            .sum()
    }
}

pub fn part2(input_map: &str) -> i32 {
    let parts: Vec<&str> = input_map.trim().split("\n\n").collect();
    let grid: Vec<Vec<char>> = parts[0]
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let steps = parts[1].replace('\n', "");

    let mut state = State::new(&grid);

    let directions = [('<', (0, -1)), ('v', (1, 0)), ('>', (0, 1)), ('^', (-1, 0))];

    for step in steps.chars() {
        if let Some(&(_, dir)) = directions.iter().find(|&&(d, _)| d == step) {
            state.move_player(dir);
        }
    }

    state.calculate_score()
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
        let input = load_input("input/day15/example.txt");

        let result = part1(&input);
        assert_eq!(result, 10092, "Failed on example case for part1");
    }

    #[test]
    fn test_my_input_case_part1() {
        let input = load_input("input/day15/input.txt");

        let result = part1(&input);
        assert_eq!(result, 1429911, "Failed on my input case for part1");
    }

    #[test]
    fn test_my_input_case_part2() {
        let input = load_input("input/day15/input.txt");

        let result = part2(&input);
        assert_eq!(result, 1425081, "Failed on my input case for part1");
    }
}
