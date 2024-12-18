use std::collections::{HashSet, VecDeque};

pub fn parse_input(input: &str) -> (Vec<(usize, usize)>, usize) {
    let mut coordinates = Vec::new();
    let mut largest_value = 0;

    for line in input.lines() {
        let parts: Vec<usize> = line
            .trim()
            .split(',')
            .filter_map(|x| x.parse().ok()) // Parse safely and filter invalid numbers
            .collect();

        if parts.len() < 2 {
            panic!(
                "Invalid line format: each line must contain two comma-separated numbers. Got: {}",
                line
            );
        }

        let x = parts[0];
        let y = parts[1];
        largest_value = largest_value.max(x.max(y));
        coordinates.push((x, y));
    }

    (coordinates, largest_value + 1)
}

pub fn part1(input_map: &str) -> usize {
    let (coordinates, size) = parse_input(input_map);
    let blocked: HashSet<(usize, usize)> = coordinates
        .iter()
        .take(1024)
        .filter(|&&(x, y)| x < size && y < size)
        .copied()
        .collect();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((0, 0, 0));
    visited.insert((0, 0));

    while let Some((x, y, steps)) = queue.pop_front() {
        if (x, y) == (size - 1, size - 1) {
            return steps;
        }

        for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            if new_x >= 0 && new_y >= 0 {
                let new_pos = (new_x as usize, new_y as usize);
                if new_x < size as isize
                    && new_y < size as isize
                    && !blocked.contains(&new_pos)
                    && !visited.contains(&new_pos)
                {
                    visited.insert(new_pos);
                    queue.push_back((new_x as usize, new_y as usize, steps + 1));
                }
            }
        }
    }

    0 // Return 0 if no path found
}

pub fn has_path_to_end(blocked_positions: &HashSet<(usize, usize)>, size: usize) -> bool {
    let start = (0, 0);
    let end = (size - 1, size - 1);
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some((x, y)) = queue.pop_front() {
        if (x, y) == end {
            return true;
        }

        for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            if new_x >= 0 && new_y >= 0 {
                let new_pos = (new_x as usize, new_y as usize);
                if new_x < size as isize
                    && new_y < size as isize
                    && !blocked_positions.contains(&new_pos)
                    && !visited.contains(&new_pos)
                {
                    visited.insert(new_pos);
                    queue.push_back(new_pos);
                }
            }
        }
    }

    false
}

pub fn part2(input_map: &str) -> Option<String> {
    let (coordinates, size) = parse_input(input_map);
    let mut left = 0;
    let mut right = coordinates.len() - 1;
    let mut last_blocking = None;

    while left <= right {
        let mid = (left + right) / 2;
        let test_blocked: HashSet<(usize, usize)> = coordinates
            .iter()
            .take(mid + 1)
            .filter(|&&(x, y)| x < size && y < size)
            .copied()
            .collect();

        if has_path_to_end(&test_blocked, size) {
            left = mid + 1;
        } else {
            right = mid - 1;
            last_blocking = Some(mid);
        }
    }

    if let Some(last_blocking) = last_blocking {
        let mut prev_blocked: HashSet<(usize, usize)> = coordinates
            .iter()
            .take(last_blocking)
            .filter(|&&(x, y)| x < size && y < size)
            .copied()
            .collect();

        for &(x, y) in &coordinates[last_blocking..last_blocking + 2] {
            if x >= size || y >= size {
                continue;
            }
            prev_blocked.insert((x, y));
            if !has_path_to_end(&prev_blocked, size) {
                return Some(format!("{},{}", x, y));
            }
        }
    }

    None
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
        let input = load_input("input/day18/example.txt");

        let result = part1(&input);
        assert_eq!(result, 0, "Failed on example case for part1");
    }

    #[test]
    fn test_my_input_case_part1() {
        let input = load_input("input/day18/input.txt");

        let result = part1(&input);
        assert_eq!(result, 250, "Failed on my input case for part1");
    }

    #[test]
    fn test_my_input_case_part22() {
        let input = load_input("input/day18/example.txt");

        let result = part2(&input).unwrap();
        assert_eq!(result, "6,1", "Failed on my input case for part1");
    }

    #[test]
    fn test_my_input_case_part2() {
        let input = load_input("input/day18/input.txt");

        let result = part2(&input).unwrap();
        assert_eq!(result, "56,8", "Failed on my input case for part1");
    }
}
