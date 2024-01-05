use rand::prelude::*;

pub fn quicksort(mut list: Vec<u8>) -> Vec<u8> {
    if list.len() < 2 {
        return list;
    } else {
        let i = (0..list.len()).choose(&mut thread_rng()).unwrap();
        let pivot = list.swap_remove(i);

        let mut less_than_pivot = vec![];
        let mut greater_than_pivot = vec![];
        for number in list {
            if number < pivot {
                less_than_pivot.push(number);
            } else {
                greater_than_pivot.push(number);
            }
        }
        let mut sorted_left = quicksort(less_than_pivot);
        let mut sorted_right = quicksort(greater_than_pivot);

        sorted_left.push(pivot);
        sorted_left.append(&mut sorted_right);

        sorted_left
    }
}

#[test]
fn can_sort_array() {
    let result = quicksort(vec![10, 17, 3, 8, 22, 100]);
    assert_eq!(result, vec![3, 8, 10, 17, 22, 100]);
}
