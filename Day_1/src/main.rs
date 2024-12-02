use std::vec::Vec;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

    fn find_smallest_num_pos_in_list(list: &Vec<i32>, excluding_list: &Vec<usize>) -> Option<usize> {
        let mut smallest = None;
        let mut smallest_index = None;

        for (i, &value) in list.iter().enumerate() {
            if !excluding_list.contains(&i) {
                if smallest.is_none() || value < smallest.unwrap() {
                    smallest = Some(value);
                    smallest_index = Some(i);
                }
            }
        }

        smallest_index
    }

    fn get_lists_from_file() -> io::Result<(Vec<i32>, Vec<i32>)> {
        let path = Path::new("input.txt");
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        let mut list_1 = Vec::new();
        let mut list_2 = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let values: Vec<i32> = line.split_whitespace()
                                       .map(|s| s.parse().unwrap())
                                       .collect();
            if values.len() == 2 {
                list_1.push(values[0]);
                list_2.push(values[1]);
            }
        }

        Ok((list_1, list_2))
    }

    fn num_times_in_list(list: &Vec<i32>, num: i32) -> usize {
        list.iter().filter(|&&x| x == num).count()
    }

    fn main() {
        let (list_1, list_2) = get_lists_from_file().expect("Failed to read lists from file");

        println!("{:?}", list_1);

        // let mut pos_list_1: Vec<i32> = Vec::new();
        // let mut pos_list_2: Vec<i32> = Vec::new();

        // Uncomment the following lines to calculate total difference
        // let mut differences = Vec::new();
        // let mut total_difference = 0;
        // for _ in 0..list_1.len() {
        //     pos_list_1.push(find_smallest_num_pos_in_list(&list_1, &pos_list_1).unwrap());
        //     pos_list_2.push(find_smallest_num_pos_in_list(&list_2, &pos_list_2).unwrap());
        // }
        // for i in 0..pos_list_1.len() {
        //     differences.push((list_1[pos_list_1[i]] - list_2[pos_list_2[i]]).abs());
        // }
        // for diff in differences {
        //     total_difference += diff;
        // }
        // println!("{}", total_difference);

        // Calculate similarity score
        let mut similarity_score = 0;
        for (i, &value) in list_1.iter().enumerate() {
            println!("{}%", (i as f64 / list_1.len() as f64) * 100.0);
            let times_in_list = num_times_in_list(&list_2, value);
            similarity_score += times_in_list as i32 * value;
        }

        println!("{}", similarity_score);
    }