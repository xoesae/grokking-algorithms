fn search_lowest(vec: &Vec<i32>) -> usize {
    let mut index = 0;
    let mut lowest = vec[index];

    for i in index..(vec.len()) {
        if vec[i] < lowest {
            index = i;
            lowest = vec[index];
        }
    }

    index
}

#[allow(dead_code)]
pub fn selectsort(mut vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();

    for _i in 0..(vec.len()) {
        let lowest = search_lowest(&vec);
        new_vec.push(vec.remove(lowest))
    }

    new_vec
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let result = selectsort([10, 11, 3, 1, 40, 30, 20].to_vec());

        assert_eq!(result, [1, 3, 10, 11, 20, 30, 40].to_vec());
    }
}