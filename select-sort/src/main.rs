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

fn select_sort(mut vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();

    for _i in 0..(vec.len()) {
        let lowest = search_lowest(&vec);
        new_vec.push(vec.remove(lowest))
    }

    new_vec
}

fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    for i in [10, 11, 3, 1, 40, 30, 20]  {
        numbers.push(i);
    }

    println!("{:?}", select_sort(numbers));
}
