pub mod selection_sort;

#[cfg(test)]
mod tests {
    use crate::selection_sort::selection_sort;

    #[test]
    fn sort_array() {
        let array = vec![5,5,4,2,9,8,1];
        assert_eq!(selection_sort(array), vec![1,2,4,5,5,8,9]);
    }
}
