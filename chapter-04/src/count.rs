pub fn count(list: Vec<u8>) -> u8 {
    list.iter().fold(0, |acc, _| acc + 1)
}

pub fn recursive_count(mut list: Vec<u8>) -> u8 {
    if list.len() == 0 {
        return 0;
    } else {
        list.remove(0);
        return 1 + recursive_count(list);
    }
}

#[test]
fn can_count_numbers() {
    let result = count(vec![1, 2, 3, 4, 5]);
    assert_eq!(result, 5);
}

#[test]
fn can_count_recursive() {
    let result = recursive_count(vec![1, 2, 3, 4, 5]);
    assert_eq!(result, 5);
}
