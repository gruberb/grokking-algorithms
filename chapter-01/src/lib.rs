pub mod binary;

#[cfg(test)]
mod test {
    use crate::binary::binary_search;
    
    #[test]
    fn test_binary_search() {
        let list = vec![1,2,3,4,5,6,7,8,9,10];
        let item = 5;

        assert_eq!(binary_search(&list, &item), Some(4));
    }
}

