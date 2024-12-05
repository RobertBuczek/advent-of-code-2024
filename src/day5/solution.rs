use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Order {
    Before,
    After,
    Unknown,
}

fn parse_input(input: &str) -> (HashMap<i32, HashMap<i32, Order>>, Vec<&str>) {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let rules = parse_rules(sections[0]);
    let data = sections[1].lines().collect();
    (rules, data)
}

pub fn part1(input: &str) -> i32 {
    let (rules, data) = parse_input(input);
    data.iter()
        .filter_map(|line| {
            let parts = parse_parts(line);
            let print = parse_print(&parts);
            if check_print(&print, &rules) {
                Some(parts[(parts.len() - 1) / 2])
            } else {
                None
            }
        })
        .sum()
}

fn parse_parts(line: &str) -> Vec<i32> {
    line.split(',')
        .map(|part| part.parse::<i32>().expect("Invalid number in input line"))
        .collect()
}

fn solve_part2(data: &str) -> i32 {
    let (rules, lines) = parse_input(data); // Parse input data into rules and lines

    lines
        .iter()
        .map(|line| {
            let parts: Vec<i32> = line.split(",").map(|part| part.parse().unwrap()).collect();
            let print = parse_print(&parts);

            if !check_print(&print, &rules) {
                let fixed = fix_print(&parts, &rules);
                let fixed_parts: Vec<i32> = fixed
                    .as_str()
                    .split(",")
                    .map(|part| part.parse().unwrap())
                    .collect();

                fixed_parts[(fixed_parts.len() - 1) / 2]
            } else {
                0
            }
        })
        .sum()
}

fn parse_rules(data: &str) -> HashMap<i32, HashMap<i32, Order>> {
    let mut map: HashMap<i32, HashMap<i32, Order>> = HashMap::new();

    for line in data.lines() {
        let split: Vec<&str> = line.split("|").collect();
        let before: i32 = split[0].parse::<i32>().unwrap();
        let after: i32 = split[1].parse::<i32>().unwrap();

        map.entry(before)
            .or_insert(HashMap::new())
            .insert(after, Order::After);
        map.entry(after)
            .or_insert(HashMap::new())
            .insert(before, Order::Before);
    }

    map
}

fn parse_print(parts: &Vec<i32>) -> HashMap<i32, HashMap<i32, Order>> {
    let mut map: HashMap<i32, HashMap<i32, Order>> = HashMap::new();

    for (current, number) in parts.iter().enumerate() {
        for other_index in 0..parts.len() {
            if current == other_index {
                continue;
            } else if other_index >= current {
                map.entry(*number)
                    .or_insert(HashMap::new())
                    .insert(parts[other_index], Order::After);
            } else {
                map.entry(*number)
                    .or_insert(HashMap::new())
                    .insert(parts[other_index], Order::Before);
            }
        }
    }

    return map;
}

fn check_print(
    print: &HashMap<i32, HashMap<i32, Order>>,
    rules: &HashMap<i32, HashMap<i32, Order>>,
) -> bool {
    let empty: HashMap<i32, Order> = HashMap::new();
    for (num, rule) in print.iter() {
        for (other_index, order) in rule.iter() {
            let thing = rules
                .get(num)
                .unwrap_or(&empty)
                .get(other_index)
                .unwrap_or(&Order::Unknown);
            if thing != &Order::Unknown && thing != order {
                return false;
            }
        }
    }

    true
}
fn fix_print(nums: &[i32], rules: &HashMap<i32, HashMap<i32, Order>>) -> String {
    let empty: HashMap<i32, Order> = HashMap::new();
    let mut weights: HashMap<i32, i32> = HashMap::new();

    for &first in nums {
        for &second in nums {
            if first == second {
                continue;
            }

            let order = rules
                .get(&first)
                .unwrap_or(&empty)
                .get(&second)
                .unwrap_or(&Order::Unknown);

            let entry = weights.entry(first).or_insert(0);
            match order {
                Order::After => *entry -= 1,
                Order::Before => *entry += 1,
                Order::Unknown => {}
            }
        }
    }

    let mut sorted: Vec<_> = weights.into_iter().collect();
    sorted.sort_by(|a, b| a.1.cmp(&b.1));

    sorted
        .iter()
        .map(|(num, _)| num.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn load_input(path: &str) -> String {
        fs::read_to_string(path).expect("Failed to read input file")
    }

    #[test]
    fn test_example_case() {
        let input = load_input("input/day5/example.txt");
        let result = part1(&input);
        assert_eq!(result, 143);
    }

    #[test]
    fn test_my_input_example_case() {
        let input = load_input("input/day5/input.txt");
        let result = part1(&input);
        assert_eq!(result, 5268);
    }

    #[test]
    fn test_multiple_rules() {
        let input = "1|2\n2|3\n\n1,2,3";
        let result = part1(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_empty_input() {
        let input = "\n\n";
        let result = part1(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_invalid_rule() {
        let input = "1|2\n\n2,1";
        let result = part1(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_large_number_of_rules() {
        let input = (1..100)
            .map(|i| format!("{}|{}", i, i + 1))
            .collect::<Vec<String>>()
            .join("\n")
            + "\n\n"
            + &(1..100)
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(",");
        let result = part1(&input);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_single_item() {
        let input = "1|2\n\n1";
        let result = part1(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_order_violation() {
        let input = "1|2\n\n2,1";
        let result = part1(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_example_case_part_2() {
        let input =
            fs::read_to_string("input/day5/example.txt").expect("Failed to read input file");

        let result = solve_part2(&input);
        assert_eq!(result, 123);
    }

    #[test]
    fn test_my_input_case_part_2() {
        let input = fs::read_to_string("input/day5/input.txt").expect("Failed to read input file");

        let result = solve_part2(&input);
        assert_eq!(result, 5799);
    }
}
