pub fn part1(input: &str) -> usize {
    let filesystem = |disk_map: &str| -> Vec<Option<usize>> {
        let mut blocks = Vec::new();
        let mut is_file = true;
        let mut id = 0;

        for x in disk_map.chars() {
            let count = x.to_digit(10).unwrap() as usize;
            if is_file {
                blocks.extend(vec![Some(id); count]);
                id += 1;
                is_file = false;
            } else {
                blocks.extend(vec![None; count]);
                is_file = true;
            }
        }

        blocks
    }(input);

    let moved_filesystem = |mut arr: Vec<Option<usize>>| -> Vec<Option<usize>> {
        let mut first_free = 0;
        while arr[first_free].is_some() {
            first_free += 1;
        }

        let mut i = arr.len() - 1;
        while arr[i].is_none() {
            i -= 1;
        }

        while i > first_free {
            arr[first_free] = arr[i];
            arr[i] = None;

            while arr[i].is_none() {
                i -= 1;
            }
            while arr[first_free].is_some() {
                first_free += 1;
            }
        }

        arr
    }(filesystem);

    let result = checksum(&moved_filesystem);
    result
}

fn checksum(arr: &[Option<usize>]) -> usize {
    arr.iter()
        .enumerate()
        .filter_map(|(i, &x)| x.map(|value| i * value))
        .sum()
}

fn part2(input: &str) -> usize {
    fn make_filesystem(
        diskmap: &str,
        size: &mut Vec<usize>,
        loc: &mut Vec<usize>,
    ) -> Vec<Option<usize>> {
        let mut blocks = Vec::new();
        let mut is_file = true;
        let mut id = 0;

        for x in diskmap.chars() {
            let x = x.to_digit(10).unwrap() as usize;
            if is_file {
                loc[id] = blocks.len();
                size[id] = x;
                blocks.extend(vec![Some(id); x]);
                id += 1;
                is_file = false;
            } else {
                blocks.extend(vec![None; x]);
                is_file = true;
            }
        }

        blocks
    }

    fn move_blocks(
        arr: &mut Vec<Option<usize>>,
        size: &[usize],
        loc: &mut Vec<usize>,
    ) -> Vec<Option<usize>> {
        // Find the last file with a positive size
        let mut big = 0;
        while size[big] > 0 {
            big += 1;
        }
        big -= 1;

        for to_move in (0..=big).rev() {
            // Find the first free space that works
            let mut free_space = 0;
            let mut first_free = 0;

            while first_free < loc[to_move] && free_space < size[to_move] {
                first_free += free_space;
                free_space = 0;

                while first_free < arr.len() && arr[first_free].is_some() {
                    first_free += 1;
                }
                while first_free + free_space < arr.len() && arr[first_free + free_space].is_none()
                {
                    free_space += 1;
                }
            }

            if first_free >= loc[to_move] {
                continue;
            }

            for idx in first_free..(first_free + size[to_move]) {
                arr[idx] = Some(to_move);
            }
            for idx in loc[to_move]..(loc[to_move] + size[to_move]) {
                arr[idx] = None;
            }
            loc[to_move] = first_free;
        }

        arr.clone()
    }

    let line = input.trim();

    let mut size = vec![0; line.len()];
    let mut loc = vec![0; line.len()];

    let mut filesystem = make_filesystem(line, &mut size, &mut loc);
    let moved_filesystem = move_blocks(&mut filesystem, &size, &mut loc);
    let result = checksum(&moved_filesystem);

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
        let input = load_input("input/day9/example.txt");

        let result = part1(&input);
        assert_eq!(result, 1928, "Failed on example case for part1");
    }

    #[test]
    fn test_my_input_case_part1() {
        let input = load_input("input/day9/input.txt");

        let result = part1(&input);
        assert_eq!(result, 6353658451014, "Failed on my input case for part1");
    }

    #[test]
    fn test_example_input_case_part2() {
        let input = load_input("input/day9/example.txt");

        let result = part2(&input);
        assert_eq!(result, 2858, "Failed on example case for part2");
    }

    #[test]
    fn test_my_input_case_part2() {
        let input = load_input("input/day9/input.txt");

        let result = part2(&input);
        assert_eq!(result, 6382582136592, "Failed on my input case for part2");
    }
}
