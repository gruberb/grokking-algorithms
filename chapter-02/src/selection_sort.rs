pub fn find_smallest(list: &[u8]) -> usize {
    list.iter().enumerate().fold(0, |smallest_index, (index, &element)| {
        if element < list[smallest_index] {
            index
        } else {
            smallest_index
        }
    })
}

pub fn selection_sort(mut list: Vec<u8>) -> Vec<u8> {
    let mut array = vec![];

    while !list.is_empty() {
        let smallest = find_smallest(&list);
        array.push(list.remove(smallest));
    }

    array
}
