pub fn maximum(list: Vec<u8>) -> u8 {
    list.iter().fold(0, |mut acc, number| {
        if number > &acc {
            acc = *number;
        }
        acc
    })
}

#[test]
fn can_find_maximum() {
    let result = maximum(vec![1, 2, 3, 4, 5]);
    assert_eq!(result, 5);
}
