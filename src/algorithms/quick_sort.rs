fn partition(arr: &mut [u32]) -> usize {
    let pivot_index = arr.len() / 2;
    arr.swap(pivot_index, arr.len() - 1);
    let mut i = 0;

    for j in 0..(arr.len() - 1) {
        if arr[j] <= arr[arr.len() - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, arr.len() - 1);

    i
}

#[allow(dead_code)]
pub fn quicksort(arr: &mut [u32]) {
    if arr.len() <= 1 { return };

    let pivot_index = partition(arr);

    quicksort(&mut arr[..pivot_index]);
    quicksort(&mut arr[(pivot_index + 1)..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut arr = [10, 11, 3, 1, 40, 30, 20];

        quicksort(&mut arr);

        assert_eq!(arr, [1, 3, 10, 11, 20, 30, 40]);
    }
}