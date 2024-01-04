pub fn binary_search(list: &[u8], item: &u8) -> Option<usize> {
    let mut low = 0;
    let mut high = list.len() -1;
    let mut sorted_list = list.to_vec();
    sorted_list.sort();

    while low <= high {
        let mid = (low + high) / 2;
        let guess = sorted_list[mid];

        if guess == *item {
            return Some(mid);
        }

        if guess > *item {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    None
}
