use std::{fs::{OpenOptions}, path::PathBuf};

use clap::ArgMatches;

pub fn insertion_sort(array: &mut Vec<i32>) -> Vec<i32> {
    for i in 1..array.len() {
        let mut j = i;
        while j > 0 && array[j - 1] > array[j] {
            array.swap(j - 1, j);
            j -= 1;
        }
    }
    array.to_vec()
}

pub fn arg_list(cmd: &ArgMatches) -> Vec<i32> {
    let mut nums: Vec<i32> = cmd
            .get_many::<String>("list")
        .expect("get nums")
            .map(|x| x.parse::<i32>().expect("error parsing to i32"))
            .collect();

    insertion_sort(&mut nums)
        }
        println!(" ");
}


pub fn sort_file(file_path: &PathBuf){
	let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path);
	todo!()
}

#[test]
fn test_insertion_sort() {
    let mut array = vec![5, 2, 4, 6, 1, 3];
    println!("unsorted: {:?}", array);
    let sorted_array = insertion_sort(&mut array);
    println!("sorted: {:?}", sorted_array);
    assert_eq!(sorted_array, vec![1, 2, 3, 4, 5, 6]);
}
