use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> usize {
    let input = parse(input);
    let mut result = HashSet::new();

    for (&a, neighbors) in &input {
        for &i in neighbors {
            for &j in neighbors {
                if i != j && input.get(i).map_or(false, |x| x.contains(&j)) {
                    let mut triangle = vec![a, i, j];
                    triangle.sort();
                    result.insert(triangle);
                }
            }
        }
    }

    result
        .iter()
        .filter(|triangle| triangle.iter().any(|node| node.starts_with('t')))
        .count()
}

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        match line.split_once('-') {
            Some((a, b)) => {
                map.entry(a).or_default().push(b);
                map.entry(b).or_default().push(a);
            }
            None => {}
        }
    }

    map
}

pub fn part2(input: &str) -> String {
    let input: HashMap<&str, Vec<&str>> = parse(input);
    let mut result = Vec::new();

    for (&node, neighbors) in &input {
        let nbr_count = neighbors.len();
        for mask in 0..(1 << nbr_count) {
            let mut nodes = vec![node];
            for (i, &neighbor) in neighbors.iter().enumerate() {
                if (mask & (1 << i)) != 0 {
                    nodes.push(neighbor);
                }
            }

            if is_clique(&nodes, &input) && nodes.len() > result.len() {
                result = nodes;
            }
        }
    }

    result.sort();
    result.join(",")
}

fn is_clique(nodes: &[&str], adj: &HashMap<&str, Vec<&str>>) -> bool {
    let n = nodes.len();
    for i in 0..n {
        for j in (i + 1)..n {
            if !adj
                .get(nodes[i])
                .map_or(false, |neighbors| neighbors.contains(&nodes[j]))
            {
                return false;
            }
        }
    }
    true
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
        let input = load_input("input/day23/example.txt");

        let result = part1(&input);
        assert_eq!(result, 7, "Failed on example case for part1");
    }

    #[test]
    fn test_my_input_case_part1() {
        let input = load_input("input/day23/input.txt");

        let result = part1(&input);
        assert_eq!(result, 1306, "Failed on my input case for part1");
    }

    #[test]
    fn test_my_input_case_part22() {
        let input = load_input("input/day23/example.txt");

        let result = part2(&input);
        assert_eq!(result, "co,de,ka,ta", "Failed on my input case for part1");
    }

    #[test]
    fn test_my_input_case_part2() {
        let input = load_input("input/day23/input.txt");

        let result = part2(&input);
        assert_eq!(
            result, "bd,dk,ir,ko,lk,nn,ob,pt,te,tl,uh,wj,yl",
            "Failed on my input case for part1"
        );
    }
}
