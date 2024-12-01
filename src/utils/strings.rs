use std::str::FromStr;

pub fn split_input_into_vector(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|line| {
            let mut split = line
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap());
            (split.next().unwrap(), split.next().unwrap())
        })
        .collect()
}

pub fn split_array_input_into_vector<T>(input: &str) -> Vec<Vec<T>>
where
    T: FromStr,
    T::Err: std::fmt::Debug, // Ensures we can unwrap the result of `FromStr`.
{
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|item| item.parse::<T>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test {

    const EXAMPLE1: &str = "3   4\n\
                            4   3\n\
                            2   5\n\
                            1   3\n\
                            3   9\n\
                            3   3";

    #[test]
    fn test_split_array_input_into_vector_with_char() {
        let input = "a b c\nd e f\ng h i";
        let result: Vec<Vec<char>> = super::split_array_input_into_vector(input);
        assert_eq!(
            result,
            vec![
                vec!['a', 'b', 'c'],
                vec!['d', 'e', 'f'],
                vec!['g', 'h', 'i']
            ]
        );
    }

    #[test]
    fn should_correctly_parse_input() {
        super::split_input_into_vector(&EXAMPLE1);
    }

    #[test]
    fn test_split_input_into_vector_basic() {
        let input = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3";
        let expected = vec![(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)];
        assert_eq!(super::split_input_into_vector(input), expected);
    }

    #[test]
    fn test_split_input_into_vector_with_extra_spaces() {
        let input = "  3   4  \n  4  3\n  2    5\n1 3   \n  3 9  \n 3   3  ";
        let expected = vec![(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)];
        assert_eq!(super::split_input_into_vector(input), expected);
    }

    #[test]
    fn test_split_input_into_vector_single_line() {
        let input = "10 20";
        let expected = vec![(10, 20)];
        assert_eq!(super::split_input_into_vector(input), expected);
    }

    #[test]
    fn test_split_input_into_vector_empty_input() {
        let input = "";
        let expected: Vec<(i32, i32)> = vec![];
        assert_eq!(super::split_input_into_vector(input), expected);
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn test_split_input_into_vector_invalid_number() {
        let input = "3 a\n4 3";
        super::split_input_into_vector(input); // This should panic due to invalid number
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_split_input_into_vector_incomplete_pair() {
        let input = "3\n4 3";
        super::split_input_into_vector(input); // This should panic due to missing second number
    }
}
