#[allow(dead_code)]
pub fn binary_search(vec: Vec<i32>, searchable: i32) -> Option<i32> {
    let mut left = 0;
    let mut right = vec.len() -1;
    let mut middle: usize;
    let mut item: i32;

    while left <= right {
        middle = (left + right) / 2;
        item = vec[middle];

        if item == searchable {
            return Some(item);
        }

        if searchable < item  {
            right = middle -1;
        } else {
            left = middle +1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_exists() {
        let mut numbers: Vec<i32> = Vec::new();

        for i in 0..12  {
            if (i % 2) == 0 {
                numbers.push(i);
            }
        }

        let result = binary_search(numbers, 4);

        assert!(result.is_some());
        assert_eq!(result.unwrap(), 4);
    }

    #[test]
    fn test_item_not_exists() {
        let mut numbers: Vec<i32> = Vec::new();

        for i in 0..12  {
            if (i % 2) == 0 {
                numbers.push(i);
            }
        }

        let result = binary_search(numbers, 11);

        assert!(result.is_none());
    }
}