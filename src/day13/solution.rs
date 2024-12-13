pub fn part1(input_map: &str) -> i32 {
    let mut ans = 0;

    let mut cura: Option<(i32, i32)> = None;
    let mut curb: Option<(i32, i32)> = None;

    let mut st = 0;
    for line in input_map.lines() {
        if line.trim().is_empty() {
            st = 0;
            continue;
        }

        if st == 0 {
            let toks: Vec<&str> = line.split_whitespace().collect();
            let x: i32 = toks[2][2..toks[2].len() - 1].parse().unwrap();
            let y: i32 = toks[3][2..].parse().unwrap();
            cura = Some((x, y));
            st = 1;
        } else if st == 1 {
            let toks: Vec<&str> = line.split_whitespace().collect();
            let x: i32 = toks[2][2..toks[2].len() - 1].parse().unwrap();
            let y: i32 = toks[3][2..].parse().unwrap();
            curb = Some((x, y));
            st = 2;
        } else {
            let toks: Vec<&str> = line.split_whitespace().collect();
            let x: i32 = toks[1][2..toks[1].len() - 1].parse().unwrap();
            let y: i32 = toks[2][2..].parse().unwrap();

            let mut score = 1_000_000;
            for i in 0..=100 {
                for j in 0..=100 {
                    if let (Some(cura), Some(curb)) = (cura, curb) {
                        if i * cura.0 + j * curb.0 == x && i * cura.1 + j * curb.1 == y {
                            score = score.min(3 * i + j);
                        }
                    }
                }
            }

            if score < 1_000_000 {
                ans += score;
            }
            st = 0;
        }
    }

    ans
}

pub fn part2(input_map: &str) -> i64 {
    let mut lines = input_map.lines();

    let mut cura: Option<(i64, i64)> = None;
    let mut curb: Option<(i64, i64)> = None;
    let mut ans: i64 = 0;
    let mut st = 0;

    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            st = 0;
            continue;
        }

        let toks: Vec<&str> = line.split_whitespace().collect();
        if st == 0 {
            let x = toks[2][2..toks[2].len() - 1].parse::<i64>().unwrap();
            let y = toks[3][2..].parse::<i64>().unwrap();
            cura = Some((x, y));
            st = 1;
        } else if st == 1 {
            let x = toks[2][2..toks[2].len() - 1].parse::<i64>().unwrap();
            let y = toks[3][2..].parse::<i64>().unwrap();
            curb = Some((x, y));
            st = 2;
        } else {
            let x = toks[1][2..toks[1].len() - 1].parse::<i64>().unwrap() + 10_000_000_000_000;
            let y = toks[2][2..].parse::<i64>().unwrap() + 10_000_000_000_000;

            if let (Some((a, c)), Some((b, d))) = (cura, curb) {
                let denom = a * d - b * c;
                if denom != 0 {
                    let i_num = d * x - b * y;
                    let j_num = -c * x + a * y;

                    if i_num % denom == 0 && j_num % denom == 0 {
                        let i = i_num / denom;
                        let j = j_num / denom;

                        if i > 0 && j > 0 {
                            ans += 3 * i + j;
                        }
                    }
                }
            }
        }
    }

    ans
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
        let input = load_input("input/day13/example.txt");

        let result = part1(&input);
        assert_eq!(result, 480, "Failed on example case for part1");
    }

    #[test]
    fn test_my_input_case_part1() {
        let input = load_input("input/day13/input.txt");

        let result = part1(&input);
        assert_eq!(result, 29877, "Failed on my input case for part1");
    }

    #[test]
    fn test_example_case_part2() {
        let input = load_input("input/day13/example.txt");

        let result = part2(&input);
        assert_eq!(result, 875318608908, "Failed on my input case for part1");
    }

    #[test]
    fn test_my_input_case_part2() {
        let input = load_input("input/day13/input.txt");

        let result = part2(&input);
        assert_eq!(result, 99423413811305, "Failed on my input case for part1");
    }
}
