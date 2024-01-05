pub fn sum(list: Vec<u8>) -> u8 {
    list.iter().fold(0, |mut acc, number| {
        acc += number;
        acc
    })
}

#[test]
fn can_sum_list_of_numbers() {
    let result = sum(vec![1, 2, 3, 4, 5]);
    assert_eq!(result, 15);
}
