struct Input {
    expected: i64,
    values: Vec<i64>,
}

enum Operation {
    Add,
    Multiply,
    Concatenate,
}

impl Operation {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
            Operation::Concatenate => format!("{}{}", a, b).parse::<i64>().unwrap_or_default(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(':');
            let expected: i64 = parts.next().unwrap().trim().parse::<i64>().unwrap();
            let values: Vec<i64> = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect();
            Input { expected, values }
        })
        .collect()
}

pub fn part1(input: &String) -> i64 {
    let data = parse_input(input);

    <[Input]>::iter(&data)
        .filter(|x| can_create_value_by_addition_multiplication(x, 0, *x.values.first().unwrap()))
        .map(|x| x.expected)
        .sum()
}

pub fn part2(input: &String) -> i64 {
    let data = parse_input(input);

    <[Input]>::iter(&data)
        .filter(|x| {
            can_create_value_by_addition_multiplication_concatenation(
                x,
                0,
                *x.values.first().unwrap(),
            )
        })
        .map(|x| x.expected)
        .sum()
}

fn can_create_value_by_addition_multiplication_concatenation(
    input: &Input,
    i: usize,
    current_value: i64,
) -> bool {
    if i + 1 >= input.values.len() {
        return current_value == input.expected;
    }

    let next_value = input.values[i + 1];

    [Operation::Add, Operation::Multiply, Operation::Concatenate]
        .iter()
        .any(|op| {
            let new_value = op.apply(current_value, next_value);
            can_create_value_by_addition_multiplication_concatenation(input, i + 1, new_value)
        })
}

fn can_create_value_by_addition_multiplication(
    input: &Input,
    i: usize,
    current_value: i64,
) -> bool {
    if i + 1 >= input.values.len() {
        return current_value == input.expected;
    }

    let next_value = input.values[i + 1];

    [Operation::Add, Operation::Multiply].iter().any(|op| {
        let new_value = op.apply(current_value, next_value);
        can_create_value_by_addition_multiplication(input, i + 1, new_value)
    })
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
        let input = load_input("input/day7/example.txt");

        let result = part1(&input);
        assert_eq!(result, 3749, "Failed on example case for part1");
    }

    #[test]
    fn test_my_input_case_part1() {
        let input = load_input("input/day7/input.txt");

        let result = part1(&input);
        assert_eq!(result, 882304362421, "Failed on my input case for part1");
    }

    #[test]
    fn test_example_case_part2() {
        let input = load_input("input/day7/example.txt");

        let result = part2(&input);
        assert_eq!(result, 11387, "Failed on example case for part2");
    }

    #[test]
    fn test_my_input_case_part2() {
        let input = load_input("input/day7/input.txt");

        let result = part2(&input);
        assert_eq!(result, 145149066755184, "Failed on my input case for part2");
    }

    #[test]
    fn test_parse_input() {
        let input = "10: 1 2 3\n15: 5 5\n20: 10 2\n";
        let result = parse_input(input);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].expected, 10);
        assert_eq!(result[0].values, vec![1, 2, 3]);
        assert_eq!(result[1].expected, 15);
        assert_eq!(result[1].values, vec![5, 5]);
        assert_eq!(result[2].expected, 20);
        assert_eq!(result[2].values, vec![10, 2]);
    }

    #[test]
    fn test_part1() {
        let input = String::from("7: 1 2 3\n6: 2 3\n10: 1 2 3\n");
        let result = part1(&input);
        assert_eq!(result, 6, "Failed on part 1 for part1");
    }

    #[test]
    fn test_part2() {
        let input = String::from("123: 1 2 3\n26: 2 3\n15: 1 5\n");
        let result = part2(&input);
        assert_eq!(result, 138, "Failed on example case for part2");
    }
}
