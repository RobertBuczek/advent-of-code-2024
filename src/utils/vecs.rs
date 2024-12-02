pub fn remove_at_index(levels: &Vec<i32>, index: usize) -> Vec<i32> {
    levels
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| i != index)
        .map(|(_, val)| *val)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_at_index_middle() {
        let levels = vec![1, 2, 3, 4, 5];
        let result = remove_at_index(&levels, 2);
        assert_eq!(
            result,
            vec![1, 2, 4, 5],
            "Removing the middle element should work"
        );
    }

    #[test]
    fn test_remove_at_index_first() {
        let levels = vec![1, 2, 3, 4, 5];
        let result = remove_at_index(&levels, 0);
        assert_eq!(
            result,
            vec![2, 3, 4, 5],
            "Removing the first element should work"
        );
    }

    #[test]
    fn test_remove_at_index_last() {
        let levels = vec![1, 2, 3, 4, 5];
        let result = remove_at_index(&levels, 4);
        assert_eq!(
            result,
            vec![1, 2, 3, 4],
            "Removing the last element should work"
        );
    }

    #[test]
    fn test_remove_at_index_out_of_bounds() {
        let levels = vec![1, 2, 3, 4, 5];
        let result = remove_at_index(&levels, 10);
        assert_eq!(
            result,
            vec![1, 2, 3, 4, 5],
            "An out-of-bounds index should not remove anything"
        );
    }

    #[test]
    fn test_remove_at_index_empty_vec() {
        let levels: Vec<i32> = vec![];
        let result = remove_at_index(&levels, 0);
        assert_eq!(
            result,
            vec![],
            "Removing from an empty vector should return an empty vector"
        );
    }

    #[test]
    fn test_remove_at_index_single_element() {
        let levels = vec![42];
        let result = remove_at_index(&levels, 0);
        assert_eq!(
            result,
            vec![],
            "Removing the only element should return an empty vector"
        );
    }
}
