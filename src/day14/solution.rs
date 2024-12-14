use std::collections::HashMap;

struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Robot {
    fn move_robot(&mut self, width: i32, height: i32) {
        self.x = (self.x + self.dx).rem_euclid(width);
        self.y = (self.y + self.dy).rem_euclid(height);
    }

    fn move_robot_step(&self, steps: i32, width: i32, height: i32) -> (i32, i32) {
        let nx = (self.x + steps * self.dx).rem_euclid(width);
        let ny = (self.y + steps * self.dy).rem_euclid(height);
        (nx, ny)
    }

    fn new(px: i32, py: i32, vx: i32, vy: i32) -> Self {
        Self {
            x: px,
            y: py,
            dx: vx,
            dy: vy,
        }
    }
}

pub fn part1(input_map: &str) -> i32 {
    const W: i32 = 101;
    const H: i32 = 103;

    let robots: Vec<Robot> = input_map.lines().map(|line| parse_input(line)).collect();
    let mut ans = vec![0, 0, 0, 0];

    for robot in robots {
        let (nx, ny) = robot.move_robot_step(100, W, H);

        if nx == W / 2 || ny == H / 2 {
            continue;
        }

        if nx < W / 2 && ny < H / 2 {
            ans[0] += 1;
        } else if nx > W / 2 && ny < H / 2 {
            ans[1] += 1;
        } else if nx < W / 2 && ny > H / 2 {
            ans[2] += 1;
        } else {
            ans[3] += 1;
        }
    }

    ans[0] * ans[1] * ans[2] * ans[3]
}

fn parse_input(line: &str) -> Robot {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let pos: Vec<i32> = parts[0][2..]
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let vel: Vec<i32> = parts[1][2..]
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    Robot::new(pos[0], pos[1], vel[0], vel[1])
}

pub fn part2(input_map: &str) -> i32 {
    let mut robots: Vec<Robot> = input_map.lines().map(|line| parse_input(line)).collect();
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;
    const SEARCH_STRING: &str = "########";

    let mut tick = 0;

    loop {
        tick += 1;

        for robot in robots.iter_mut() {
            robot.move_robot(WIDTH, HEIGHT);
        }

        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        for robot in robots.iter() {
            map.entry(robot.y).or_insert_with(Vec::new).push(robot.x);
        }

        for (.., x_positions) in map.iter() {
            if x_positions.len() < SEARCH_STRING.len() {
                continue;
            }

            let mut row = vec!['.'; WIDTH as usize];
            for &x in x_positions {
                row[x as usize] = '#';
            }
            if row.iter().collect::<String>().contains(SEARCH_STRING) {
                return tick;
            }
        }
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
        let input = load_input("input/day14/example.txt");

        let result = part1(&input);
        assert_eq!(result, 21, "Failed on example case for part1");
    }

    #[test]
    fn test_my_input_case_part1() {
        let input = load_input("input/day14/input.txt");

        let result = part1(&input);
        assert_eq!(result, 218433348, "Failed on my input case for part1");
    }

    #[test]
    fn test_my_input_case_part2() {
        let input = load_input("input/day14/input.txt");

        let result = part2(&input);
        assert_eq!(result, 6512, "Failed on my input case for part1");
    }
}
