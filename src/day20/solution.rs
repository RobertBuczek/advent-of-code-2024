pub fn part1(input_map: &str) -> usize {
    let input: Vec<Vec<usize>> = match parse_input(input_map) {
        Ok(input) => input,
        Err(_) => {
            return 0;
        }
    };
    cheats_that_save_at_least(&input, 100, 2)
}

pub fn part2(input_map: &str) -> usize {
    let input: Vec<Vec<usize>> = match parse_input(input_map) {
        Ok(input) => input,
        Err(_) => {
            return 0;
        }
    };
    cheats_that_save_at_least(&input, 100, 20)
}

fn parse_input(text: &str) -> Result<Vec<Vec<usize>>, String> {
    text.lines().map(parse_row).collect()
}

/// Helper function to parse a single row from the input.
fn parse_row(line: &str) -> Result<Vec<usize>, String> {
    line.chars().map(parse_char).collect()
}

/// Helper function to parse a single character into its corresponding usize value.
fn parse_char(ch: char) -> Result<usize, String> {
    match ch {
        '.' => Ok(0),
        '#' => Ok(1),
        'S' => Ok(2),
        'E' => Ok(3),
        _ => Err(format!("Invalid character in input: {}", ch)),
    }
}

fn cheats_that_save_at_least(input: &Vec<Vec<usize>>, floor: usize, duration: usize) -> usize {
    let nr = input.len();
    let nc = input[0].len();

    let mut map = input.clone();
    let mut start = None;
    let mut end = None;

    // Iterate over the grid and process special cells
    for (r, row) in map.iter_mut().enumerate() {
        for (c, ch) in row.iter_mut().enumerate() {
            match *ch {
                1 => {
                    *ch = usize::MAX;
                }
                2 => {
                    *ch = 0;
                    if start.is_none() {
                        start = Some((r, c));
                    }
                }
                3 => {
                    *ch = 0;
                    if end.is_none() {
                        end = Some((r, c));
                    }
                }
                _ => {}
            }
        }
    }

    // Ensure start and end are found, with default values if not.
    let start = start.unwrap_or((0, 0)); // Handle as needed for missing start
    let end = end.unwrap_or((0, 0)); // Handle as needed for missing end

    let mut point = end;
    let mut path = vec![end];

    // Backtrack from end to start
    while point != start {
        let mut n = 0;
        for (rr, cc) in neighbors(nr, nc, point.0, point.1) {
            if map[rr][cc] == 0 && (rr, cc) != end {
                point = (rr, cc);
                n = 1;
                map[rr][cc] = path.len();
                path.push((rr, cc));
            }
        }

        // If no valid neighbor found, return early
        if n == 0 {
            return 0;
        }
    }

    let track_len = path.len() - 1;
    let threshold = track_len - floor;

    path.reverse(); // Reverse path for easier processing later

    let mut count = 0;

    // Process the path to check for cheats
    for (i, point) in path.iter().copied().enumerate() {
        let (r, c) = point;

        for (rr, cc) in cheats(nr, nc, r, c, duration) {
            if map[rr][cc] < usize::MAX {
                let t = i + r.abs_diff(rr) + c.abs_diff(cc) + map[rr][cc];

                if t <= threshold {
                    count += 1;
                }
            }
        }
    }

    count
}

fn neighbors(nr: usize, nc: usize, r: usize, c: usize) -> impl Iterator<Item = (usize, usize)> {
    let potential_neighbors = [
        (r, c + 1),             // right
        (r.wrapping_sub(1), c), // up
        (r, c.wrapping_sub(1)), // left
        (r + 1, c),             // down
    ];

    potential_neighbors
        .into_iter()
        .filter(move |&(rr, cc)| is_valid_cell(nr, nc, rr, cc))
}

/// Helper function to check if a given cell (rr, cc) is within grid bounds.
fn is_valid_cell(nr: usize, nc: usize, r: usize, c: usize) -> bool {
    r < nr && c < nc
}

fn cheats(
    nr: usize,
    nc: usize,
    r: usize,
    c: usize,
    duration: usize,
) -> impl Iterator<Item = (usize, usize)> {
    // Calculate valid row range, ensuring it's within grid bounds.
    let row_range = row_range(nr, r, duration);

    // For each row in the valid range, calculate the valid column range.
    row_range.flat_map(move |rr| {
        // Calculate the safe column range based on the row distance.
        let column_range = column_range(nc, c, duration, r.abs_diff(rr));
        column_range.map(move |cc| (rr, cc))
    })
}

/// Helper function to calculate the valid row range.
fn row_range(nr: usize, r: usize, duration: usize) -> impl Iterator<Item = usize> {
    r.saturating_sub(duration)..=(r + duration).min(nr - 1)
}

/// Helper function to calculate the valid column range.
fn column_range(
    nc: usize,
    c: usize,
    duration: usize,
    row_diff: usize,
) -> impl Iterator<Item = usize> {
    let dc = duration.saturating_sub(row_diff); // Safeguard column range based on row distance
    c.saturating_sub(dc)..=(c + dc).min(nc - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn load_input(file_path: &str) -> String {
        fs::read_to_string(file_path).expect(&format!("Failed to read input file: {}", file_path))
    }

    #[test]
    fn test_my_input_case_part1() {
        let input = load_input("input/day20/input.txt");

        let result = part1(&input);
        assert_eq!(result, 1459, "Failed on my input case for part1");
    }

    #[test]
    fn test_my_input_case_part2() {
        let input = load_input("input/day20/input.txt");

        let result = part2(&input);
        assert_eq!(result, 1016066, "Failed on my input case for part1");
    }
}
