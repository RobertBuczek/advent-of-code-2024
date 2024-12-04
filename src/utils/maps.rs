use std::collections::HashMap;
use std::hash::Hash;

pub fn group_and_count(vec: Vec<i32>) -> HashMap<i32, i32> {
    let mut counts = HashMap::new();

    for &item in vec.iter() {
        *counts.entry(item).or_insert(0) += 1;
    }

    counts
}

pub fn group_and_count_by_type<T>(vec: Vec<T>) -> HashMap<T, i32>
where
    T: Eq + Hash,
{
    let mut counts: HashMap<T, i32> = HashMap::new();
    for item in vec {
        *counts.entry(item).or_insert(0) += 1;
    }

    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        let input = vec![1, 2, 2, 3];
        let result = group_and_count(input);

        let mut expected = HashMap::new();
        expected.insert(1, 1);
        expected.insert(2, 2);
        expected.insert(3, 1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_case() {
        let input: Vec<i32> = vec![];
        let result = group_and_count(input);

        let expected: HashMap<i32, i32> = HashMap::new();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_element() {
        let input = vec![5];
        let result = group_and_count(input);

        let mut expected = HashMap::new();
        expected.insert(5, 1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_all_unique_elements() {
        let input = vec![1, 2, 3, 4, 5];
        let result = group_and_count(input);

        let mut expected = HashMap::new();
        expected.insert(1, 1);
        expected.insert(2, 1);
        expected.insert(3, 1);
        expected.insert(4, 1);
        expected.insert(5, 1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_negative_numbers() {
        let input = vec![-1, -2, -2, -3, -3, -3];
        let result = group_and_count(input);

        let mut expected = HashMap::new();
        expected.insert(-1, 1);
        expected.insert(-2, 2);
        expected.insert(-3, 3);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_order_does_not_matter() {
        let input = vec![2, 3, 1, 2, 1, 3];
        let result = group_and_count(input);

        let mut expected = HashMap::new();
        expected.insert(1, 2);
        expected.insert(2, 2);
        expected.insert(3, 2);

        assert_eq!(result, expected);
    }
}
