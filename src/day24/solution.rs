use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> i64 {
    let sections: Vec<&str> = input.trim().split("\n\n").collect();
    let wire_lines = sections[0].lines();
    let logic_lines = sections[1].lines();

    let mut wires: HashMap<String, i32> = HashMap::new();

    // Parse wire values
    for line in wire_lines {
        let parts: Vec<&str> = line.split(": ").collect();
        let wire = parts[0].to_string();
        let value = parts[1].parse::<i32>().unwrap();
        wires.insert(wire, value);
    }

    // Process logic until all conditions are met
    loop {
        let mut good = true;

        for line in logic_lines.clone() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let w1 = parts[0];
            let op = parts[1];
            let w2 = parts[2];
            let out = parts[4];

            if !wires.contains_key(w1) || !wires.contains_key(w2) {
                good = false;
                continue;
            }

            let val1 = *wires.get(w1).unwrap();
            let val2 = *wires.get(w2).unwrap();
            let result = match op {
                "AND" => val1 & val2,
                "OR" => val1 | val2,
                "XOR" => val1 ^ val2,
                _ => continue,
            };

            wires.insert(out.to_string(), result);
        }

        if good {
            break;
        }
    }

    // Collect and process final wire values
    let mut values: Vec<(String, i32)> = wires
        .iter()
        .filter(|(k, _)| k.starts_with('z'))
        .map(|(k, &v)| (k.clone(), v))
        .collect();

    values.sort_by(|a, b| a.0.cmp(&b.0));

    let binary_string: String = values.iter().rev().map(|(_, v)| v.to_string()).collect();

    i64::from_str_radix(&binary_string, 2).unwrap()
}

struct Gate<'a> {
    a: &'a str,
    op: &'a str,
    b: &'a str,
    output: &'a str,
}

impl<'a> Gate<'a> {
    fn new(a: &'a str, op: &'a str, b: &'a str, output: &'a str) -> Gate<'a> {
        Gate { a, op, b, output }
    }

    // Method to check if the gate is direct (either a or b starts with 'x')
    fn is_direct(&self) -> bool {
        self.a.starts_with('x') || self.b.starts_with('x')
    }

    // Method to check if the gate is an output gate (output starts with 'z')
    fn is_output(&self) -> bool {
        self.output.starts_with('z')
    }

    fn check_opcode(&self, op: &str) -> bool {
        self.op == op
    }
}

fn part2(input: &str) -> String {
    let mut sections = input.trim().split("\n\n");
    let wires_raw = sections.next().unwrap().lines();
    let gates_raw = sections.next().unwrap().lines();

    // Parse wires
    let mut wires: HashMap<&str, Option<i32>> = HashMap::new();
    for line in wires_raw {
        let mut parts = line.split(": ");
        let name = parts.next().unwrap();
        let value = parts.next().unwrap().parse::<i32>().unwrap();
        wires.insert(name, Some(value));
    }

    let input_bit_count = wires.len() / 2;

    // Parse gates
    let mut gates = Vec::new();
    for line in gates_raw {
        let mut parts = line.split(" -> ");
        let inputs = parts.next().unwrap();
        let output = parts.next().unwrap();

        let mut input_parts = inputs.split(' ');
        let a = input_parts.next().unwrap();
        let op = input_parts.next().unwrap();
        let b = input_parts.next().unwrap();

        gates.push(Gate::new(a, op, b, output));

        if !wires.contains_key(a) {
            wires.insert(a, None);
        }
        if !wires.contains_key(b) {
            wires.insert(b, None);
        }
        if !wires.contains_key(output) {
            wires.insert(output, None);
        }
    }

    let mut flags: HashSet<&str> = HashSet::new();

    for gate in &gates {
        if gate.is_direct() && gate.check_opcode("XOR") {
            let is_first = gate.a == "x00" || gate.b == "x00";
            if is_first {
                if gate.output != "z00" {
                    flags.insert(gate.output);
                }
                continue;
            } else if gate.output == "z00" {
                flags.insert(gate.output);
            }

            if gate.is_output() {
                flags.insert(gate.output);
            }
        }
    }

    for gate in &gates {
        if gate.check_opcode("XOR") && !gate.is_direct() && !gate.is_output() {
            flags.insert(gate.output);
        }
    }

    // Output gates check
    for gate in &gates {
        if gate.is_output() {
            if gate.output == format!("z{:03}", input_bit_count) {
                if gate.check_opcode("OR") {
                    flags.insert(gate.output);
                }
                continue;
            } else if gate.op != "XOR" {
                flags.insert(gate.output);
            }
        }
    }

    // More complex checks
    let mut check_next = Vec::new();
    for gate in &gates {
        if gate.is_direct() && gate.check_opcode("XOR") {
            if flags.contains(gate.output) || gate.output == "z00" {
                continue;
            }

            let mut found = false;
            for check_gate in &gates {
                if check_gate.check_opcode("XOR") && !check_gate.is_direct() {
                    if check_gate.a == gate.output || check_gate.b == gate.output {
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                check_next.push(gate);
                flags.insert(gate.output);
            }
        }
    }

    for gate in &check_next {
        let intended_result = format!("z{}", &gate.a[1..]);
        let mut matches = Vec::new();

        for check_gate in &gates {
            if check_gate.check_opcode("XOR") && check_gate.output == intended_result {
                matches.push(check_gate);
            }
        }

        let match_gate = matches[0];
        let to_check = [match_gate.a, match_gate.b];

        let mut or_matches = Vec::new();
        for check_gate in &gates {
            if check_gate.check_opcode("OR") {
                if check_gate.output == to_check[0] || check_gate.output == to_check[1] {
                    or_matches.push(check_gate);
                }
            }
        }

        let correct_output = if or_matches[0].output == to_check[0] {
            to_check[1]
        } else {
            to_check[0]
        };

        flags.insert(correct_output);
    }

    flags.into_iter().sorted().collect::<Vec<_>>().join(",")
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
        let input = load_input("input/day24/example.txt");

        let result = part1(&input);
        assert_eq!(result, 4, "Failed on example case for part1");
    }

    #[test]
    fn test_my_input_case_part1() {
        let input = load_input("input/day24/input.txt");

        let result = part1(&input);
        assert_eq!(result, 55920211035878, "Failed on my input case for part1");
    }

    #[test]
    fn test_my_input_case_part2() {
        let input = load_input("input/day24/input.txt");

        let result = part2(&input);
        assert_eq!(
            result, "btb,cmv,mwp,rdg,rmj,z17,z23,z30,z45",
            "Failed on my input case for part1"
        );
    }
}
