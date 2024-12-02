use std::path::Path;
use std::vec::Vec;
use std::fs::File;
use std::io::{BufReader, BufRead};

// Read input.txt to generate lists of integers
fn get_lists() -> Vec<Vec<i32>> {
    let mut list: Vec<Vec<i32>> = Vec::new();
    let file: File = File::open(&Path::new("input.txt")).expect("Unable to open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let mut l: Vec<i32> = Vec::new();
        for num in line.unwrap().split_whitespace() {
            l.push(num.parse::<i32>().unwrap());
        }
        list.push(l);
    }

    return list;
}

fn get_list_of_differences(list: Vec<i32>) -> Vec<i32> {
    let mut differences: Vec<i32> = Vec::new();
    for i in 0..list.len() - 1 {
        differences.push(list[i + 1] - list[i]);
    }
    return differences;
}

fn is_safe(list: Vec<i32>) -> bool {
    let differences = get_list_of_differences(list);
    let mut safe: bool = true;

    for i in 0..differences.len() {
        if differences[i].abs() < 1 || differences[i].abs() > 3 {
            safe = false
        } else if differences[i] == 0 {
            safe = false
        }
    }

    let mut positive_differences: i32 = 0;
    let mut negative_differences: i32 = 0;

    for i in 0..differences.len() {
        if differences[i] > 0 {
            positive_differences += 1;
        } else if differences[i] < 0 {
            negative_differences += 1;
        }
    }

    if !((positive_differences == 0) || (negative_differences == 0)) {
        safe = false
    }
    return safe;
}

fn main() {
    let lists = get_lists();
    let mut num_safe: i32 = 0;

    for list in lists {
        // check each list with one value removed at a time for example if there is a list of [1, 2, 3, 4, 5] it will attempt with 1 removed like so [2, 3, 4, 5] then attempt with only 2 removed like so [1, 3, 4, 5]
        for i in 0..list.len() {
            let mut temp_list = list.clone();
            temp_list.remove(i);
            if is_safe(temp_list) {
                num_safe += 1;
                break;
            }
        }
    }
    println!("Number of safe lists: {}", num_safe);
}