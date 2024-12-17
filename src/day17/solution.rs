pub fn part1(input_map: &str) -> String {
    let (mut register_a, mut register_b, mut register_c, program): (i64, i64, i64, Vec<u8>) =
        parse_input(input_map);

    let mut outputs: Vec<i64> = Vec::new();
    let mut ip: usize = 0; // Instruction pointer

    while ip < program.len() {
        let opcode = program[ip];
        let operand = program[ip + 1];
        ip += 2;

        match opcode {
            0 => {
                register_a /=
                    2_i64.pow(get_combo_value(operand, register_a, register_b, register_c) as u32)
            } // adv
            1 => register_b ^= operand as i64, // bxl
            2 => register_b = get_combo_value(operand, register_a, register_b, register_c) % 8, // bst
            3 => {
                if register_a != 0 {
                    ip = operand as usize;
                }
            } // jnz
            4 => register_b ^= register_c, // bxc
            5 => outputs.push(get_combo_value(operand, register_a, register_b, register_c) % 8), // out
            6 => {
                register_b = register_a
                    / 2_i64.pow(get_combo_value(operand, register_a, register_b, register_c) as u32)
            } // bdv
            7 => {
                register_c = register_a
                    / 2_i64.pow(get_combo_value(operand, register_a, register_b, register_c) as u32)
            } // cdv
            _ => panic!("Invalid opcode!"),
        }
    }

    outputs
        .iter()
        .map(|&value| value.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn parse_input(input: &str) -> (i64, i64, i64, Vec<u8>) {
    let mut register_a = 0;
    let mut register_b = 0;
    let mut register_c = 0;
    let mut program = Vec::new();

    for line in input.lines() {
        match line.split_once(": ") {
            Some(("Register A", value)) => register_a = value.trim().parse().unwrap(),
            Some(("Register B", value)) => register_b = value.trim().parse().unwrap(),
            Some(("Register C", value)) => register_c = value.trim().parse().unwrap(),
            Some(("Program", value)) => {
                program = value
                    .split(',')
                    .map(|num| num.trim().parse().unwrap())
                    .collect()
            }
            _ => {}
        }
    }

    (register_a, register_b, register_c, program)
}

fn get_combo_value(operand: u8, a: i64, b: i64, c: i64) -> i64 {
    match operand {
        0..=3 => operand as i64,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("Invalid combo operand!"),
    }
}

pub fn part2(input: &str) -> i64 {
    let (_, register_b, register_c, program) = parse_input(input); // Unwrap the input

    // Inline search function
    fn search(a: i64, b: i64, c: i64, program: &[u8], len: usize) -> Option<i64> {
        if program.len() + 1 == len {
            return Some(a);
        }

        (0..8).find_map(|word| {
            let new_a = (a << 3) | word; // Update `a` by shifting and bitwise OR
            if run_program_state(new_a, b, c, program) == &program[program.len() - len..] {
                search(new_a, b, c, program, len + 1) // Recurse
            } else {
                None
            }
        })
    }

    search(0, register_b, register_c, &program, 1).unwrap() // Call and unwrap search result
}

fn run_program_state(mut a: i64, mut b: i64, mut c: i64, program: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    let mut ip = 0;

    while ip < program.len() {
        let opcode = program[ip];
        let operand = program.get(ip + 1).copied().unwrap_or(0); // Safely fetch operand
        ip += 2;

        match opcode {
            0 => a >>= get_combo_value(operand, a, b, c),   // adv
            1 => b ^= operand as i64,                       // bxl
            2 => b = get_combo_value(operand, a, b, c) % 8, // bst
            3 => {
                if a != 0 {
                    ip = operand as usize;
                }
            } // jnz
            4 => b ^= c,                                    // bxc
            5 => output.push((get_combo_value(operand, a, b, c) % 8) as u8), // out
            6 => b = a >> get_combo_value(operand, a, b, c), // bdv
            7 => c = a >> get_combo_value(operand, a, b, c), // cdv
            _ => panic!("Invalid opcode!"),                 // Invalid opcode handler
        }
    }

    output
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
        let input = load_input("input/day17/example.txt");

        let result = part1(&input);
        assert_eq!(
            result, "4,6,3,5,6,3,5,2,1,0",
            "Failed on example case for part1"
        );
    }

    #[test]
    fn test_my_input_case_part1() {
        let input = load_input("input/day17/input.txt");

        let result = part1(&input);
        assert_eq!(
            result, "2,0,7,3,0,3,1,3,7",
            "Failed on my input case for part1"
        );
    }

    #[test]
    fn test_example_case_part2() {
        let input = "Register A: 2024\nRegister B: 0\nRegister C: 0\nProgram: 0,3,5,4,3,0";
        let result = part2(&input);
        assert_eq!(result, 117440);
    }

    #[test]
    fn test_my_input_case_part2() {
        let input = load_input("input/day17/input.txt");

        let result = part2(&input);
        assert_eq!(result, 247839539763386, "Failed on my input case for part1");
    }
}
