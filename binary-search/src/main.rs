fn binary_search(vec: Vec<i32>, searchable: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = vec.len() -1;
    let mut middle: usize;
    let mut item: i32;

    while left <= right {
        middle = (left + right) / 2;
        item = vec[middle];

        if item == searchable {
            return Some(middle);
        }

        if searchable < item  {
            right = middle -1;
        } else {
            left = middle +1;
        }
    }

    None
}

fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    for i in 0..12  {
        if (i % 2) == 0 {
            numbers.push(i);
        }
    }

    match binary_search(numbers, 11) {
        Some(result) => println!("{result}"),
        None => println!("Not Found"),
    }
}
