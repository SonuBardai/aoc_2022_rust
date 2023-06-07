use crate::cleanup::{get_contained_pairs, get_overlapping_pairs, get_pairs};
use crate::utils::{get_all_values, read_file};

pub fn day4a_solution(input_file_name: &str) {
    let file_contents = read_file(input_file_name);
    let input_values = get_all_values(&file_contents);
    let pairs = get_pairs(input_values);
    let contained_pairs = get_contained_pairs(pairs);
    // println!("Pairs: {contained_pairs:?}");
    println!(
        "There are {} pairs where one's range fully contains the other's",
        contained_pairs.len()
    );
}

pub fn day4b_solution(input_file_name: &str) {
    let file_contents = read_file(input_file_name);
    let input_values = get_all_values(&file_contents);
    let pairs = get_pairs(input_values);
    let overlapping_pairs = get_overlapping_pairs(pairs);
    // println!("Pairs: {overlapping_pairs:?}");
    println!(
        "There are {} pairs where one's range overlaps the other's",
        overlapping_pairs.len()
    );
}
