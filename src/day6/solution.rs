use std::collections::HashSet;
#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Status {
    point: (usize, usize),
    direction: Direction,
}

pub fn part2(input: &str) -> i32 {
    let mut map = Vec::new();
    for line in input.lines() {
        map.push(line.chars().collect::<Vec<_>>());
    }

    let starting_position = find_starting_position(&map);
    let mut valid_obstructions = 0;

    let rows = map.len();
    let cols = map[0].len();

    for row in 0..rows {
        for col in 0..cols {
            if map[row][col] == '.' && (row, col) != starting_position {
                map[row][col] = '#';

                if causes_loop(&map, starting_position) {
                    valid_obstructions += 1;
                }

                map[row][col] = '.';
            }
        }
    }

    valid_obstructions
}

fn causes_loop(map: &Vec<Vec<char>>, starting_position: (usize, usize)) -> bool {
    let mut visited: HashSet<Status> = HashSet::new();
    let mut current_position = starting_position;
    let mut direction = Direction::North;

    loop {
        let status = Status {
            point: current_position,
            direction,
        };

        if visited.contains(&status) {
            return true;
        }

        visited.insert(status);

        let next_position = calculate_next_position(current_position, &direction, map);

        if next_position.is_none() {
            return false;
        }

        let (next_row, next_col) = next_position.unwrap();

        if map[next_row][next_col] == '#' {
            direction = rotate_direction(&direction);
        } else {
            current_position = (next_row, next_col);
        }
    }
}

fn calculate_next_position(
    current_position: (usize, usize),
    direction: &Direction,
    map: &Vec<Vec<char>>,
) -> Option<(usize, usize)> {
    let (row, col) = current_position;
    let rows = map.len();
    let cols = map[0].len();

    match direction {
        Direction::North => {
            if row > 0 {
                Some((row - 1, col))
            } else {
                None
            }
        }
        Direction::South => {
            if row + 1 < rows {
                Some((row + 1, col))
            } else {
                None
            }
        }
        Direction::East => {
            if col + 1 < cols {
                Some((row, col + 1))
            } else {
                None
            }
        }
        Direction::West => {
            if col > 0 {
                Some((row, col - 1))
            } else {
                None
            }
        }
    }
}

fn rotate_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

pub fn part1(input: &str) -> i32 {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let starting_position = find_starting_position(&map);
    let path = create_path(&map, starting_position);

    path.len() as i32
}

fn find_starting_position(map: &[Vec<char>]) -> (usize, usize) {
    map.iter()
        .enumerate()
        .find_map(|(x, row)| {
            row.iter()
                .enumerate()
                .find(|&(_, &ch)| ch == '^')
                .map(|(y, _)| (x, y))
        })
        .expect("No starting position ('^') found in the map")
}

fn create_path(map: &[Vec<char>], starting_point: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut path: HashSet<(usize, usize)> = HashSet::new();
    let mut current_point = starting_point;
    let mut direction = Direction::North;

    path.insert(starting_point);

    loop {
        let next_point = calculate_next_point(current_point, &direction);
        if next_point.0 >= map.len() || next_point.1 >= map[0].len() {
            break;
        }
        if map[next_point.0][next_point.1] == '#' {
            direction = rotate_direction(&direction); // Obstruction encountered, rotate direction
        } else {
            path.insert(next_point);
            current_point = next_point;
        }
    }

    path
}

fn calculate_next_point(current_point: (usize, usize), direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::North => (current_point.0.saturating_sub(1), current_point.1),
        Direction::South => (current_point.0 + 1, current_point.1),
        Direction::East => (current_point.0, current_point.1 + 1),
        Direction::West => (current_point.0, current_point.1.saturating_sub(1)),
    }
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
        let input = load_input("input/day6/example.txt");

        let result = part1(&input);
        assert_eq!(result, 41, "Failed on example case for part1");
    }

    #[test]
    fn test_my_input_case_part1() {
        let input = load_input("input/day6/input.txt");

        let result = part1(&input);
        assert_eq!(result, 5409, "Failed on my input case for part1");
    }

    #[test]
    fn test_example_case_part2() {
        let input = load_input("input/day6/example.txt");

        let result = part2(&input);
        assert_eq!(result, 6, "Failed on example case for part2");
    }

    #[test]
    fn test_my_input_case_part2() {
        let input = load_input("input/day6/input.txt");

        let result = part2(&input);
        assert_eq!(result, 2022, "Failed on my input case for part2");
    }

    #[test]
    fn test_small_input_part2() {
        let input = "^\n#\n.";
        let result = part2(input);
        assert_eq!(result, 0, "Failed on small input for part2");
    }

    #[test]
    fn test_complex_grid_part1() {
        let input = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
        let result = part1(input);
        assert_eq!(result, 41, "Failed on complex grid for part1");
    }

    #[test]
    fn test_complex_grid_part2() {
        let input = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
        let result = part2(input);
        assert_eq!(result, 6, "Failed on complex grid for part2");
    }
}
