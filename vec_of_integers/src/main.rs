use rand::Rng;
use std::collections::HashMap;

// list of integers, return the average, median when sorted and the value that occurs most often

fn main() {
    let int_vec = generate_vec(5);
    println!("vector: {:?}", int_vec);

    let average = get_average(&int_vec);
    let median = get_median(&int_vec);
    let mode = mode(&int_vec);

    println!("mean: {}", average);
    println!("median: {}", median);
    println!("mode: {:?}", mode);
}

fn generate_vec(len: i8) -> Vec<i32> {
    let mut integers: Vec<i32> = Vec::new();

    for _i in 0..len {
        integers.push(rand::thread_rng().gen_range(1, 11));
    }

    return integers;
}

fn get_average(vec: &Vec<i32>) -> i32 {
    let mut sum = 0;
    let length = vec.len() as i32;

    for i in vec {
        sum += i;
    }

    sum / length
}

fn get_median(vec: &Vec<i32>) -> i32 {
    let mut median_vec = vec.clone();

    median_vec.sort();

    println!("sorted: {:?}", median_vec);

    median_vec[median_vec.len() / 2]
}

fn mode(vec: &Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();

    // create a map of the counts
    for i in vec {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut count_map: HashMap<i32, Vec<i32>>= HashMap::new();

    // create a map of vectors with the counts
    for (key, value) in map {
        let count_vec = &mut count_map.entry(value).or_insert(Vec::new());
        count_vec.push(*key);
    }

    let max_key = count_map.keys().cloned().max().unwrap_or(0);

    // return a copy of the 
    return count_map[&max_key].clone();
}
