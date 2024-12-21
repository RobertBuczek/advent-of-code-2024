use itertools::Itertools;
use std::collections::HashMap;

type Position = (i64, i64);

const DIRECTIONS: [(char, Position); 4] =
    [('^', (-1, 0)), ('v', (1, 0)), ('<', (0, -1)), ('>', (0, 1))];

fn add_pos(p1: Position, p2: Position) -> Position {
    (p1.0 + p2.0, p1.1 + p2.1)
}

fn sub_pos(p1: Position, p2: Position) -> Position {
    (p1.0 - p2.0, p1.1 - p2.1)
}

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| line.trim().to_string())
        .map(|code| calculate_sequence_complexity(&code, 2))
        .sum()
}

pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| line.trim().to_string())
        .map(|code| calculate_sequence_complexity(&code, 25))
        .sum()
}

fn calculate_sequence_complexity(code: &str, number_of_robots: i64) -> i64 {
    let mut cache = HashMap::new();
    let length = find_shortest(code, number_of_robots, 0, &mut cache);
    length * code[..code.len() - 1].parse::<i64>().unwrap()
}


fn find_shortest(
    sequence: &str,
    number_of_robots: i64,
    depth: i64,
    cache: &mut HashMap<(String, i64, i64), i64>,
) -> i64 {
    let key = (sequence.to_string(), depth, number_of_robots);
    if let Some(&cached) = cache.get(&key) {
        return cached;
    }

    let mut positions = HashMap::new();
    positions.insert('7', (0, 0));
    positions.insert('8', (0, 1));
    positions.insert('9', (0, 2));
    positions.insert('4', (1, 0));
    positions.insert('5', (1, 1));
    positions.insert('6', (1, 2));
    positions.insert('1', (2, 0));
    positions.insert('2', (2, 1));
    positions.insert('3', (2, 2));
    positions.insert('0', (3, 1));
    positions.insert('A', (3, 2));
    positions.insert('^', (0, 1));
    positions.insert('a', (0, 2));
    positions.insert('<', (1, 0));
    positions.insert('v', (1, 1));
    positions.insert('>', (1, 2));

    let mut curr = if depth == 0 {
        positions[&'A']
    } else {
        positions[&'a']
    };
    let mut total_length = 0;

    let avoid = if depth == 0 { (3, 0) } else { (0, 0) };

    for char in sequence.chars() {
        let target = positions[&char];
        let moves = generate_moves(curr, target, avoid);
        if depth == number_of_robots {
            total_length += moves.iter().map(|m| m.len()).min().unwrap() as i64;
        } else {
            total_length += moves
                .iter()
                .map(|m| find_shortest(m, number_of_robots, depth + 1, cache))
                .min()
                .unwrap();
        }
        curr = target;
    }

    cache.insert(key, total_length);
    total_length
}

fn generate_moves(start: Position, end: Position, avoid: Position) -> Vec<String> {
    let (dy, dx) = sub_pos(end, start);
    let mut moves = Vec::new();
    if dy < 0 {
        moves.extend(vec!["^"; dy.abs() as usize]);
    } else {
        moves.extend(vec!["v"; dy as usize]);
    }
    if dx < 0 {
        moves.extend(vec!["<"; dx.abs() as usize]);
    } else {
        moves.extend(vec![">"; dx as usize]);
    }

    let mut result = Vec::new();
    for perm in moves.iter().permutations(moves.len()).unique() {
        let mut pos = start;
        let mut valid = true;
        for move_ in &perm {
            let dir = DIRECTIONS
                .iter()
                .find(|&&(k, _)| k == move_.chars().next().unwrap())
                .unwrap()
                .1;
            pos = add_pos(pos, dir);
            if pos == avoid {
                valid = false;
                break;
            }
        }
        if valid {
            result.push(format!(
                "{}a",
                perm.into_iter().cloned().collect::<String>()
            ));
        }
    }
    if result.is_empty() {
        result.push("a".to_string());
    }
    result
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
        let input = load_input("input/day21/example.txt");

        let result = part1(&input);
        assert_eq!(result, 126384, "Failed on example case for part1");
    }

    #[test]
    fn test_my_input_case_part1() {
        let input = load_input("input/day21/input.txt");

        let result = part1(&input);
        assert_eq!(result, 105458, "Failed on my input case for part1");
    }

    #[test]
    fn test_example_case_part2() {
        let input = load_input("input/day21/example.txt");

        let result = part2(&input);
        assert_eq!(result, 154115708116294, "Failed on example case for part2");
    }

    #[test]
    fn test_my_input_case_part2() {
        let input = load_input("input/day21/input.txt");

        let result = part2(&input);
        assert_eq!(result, 129551515895690, "Failed on my input case for part2");
    }
}
