use crate::rucksack::{group::Group, items::get_priority, Rucksack};
use crate::utils::{get_all_values, read_file};

pub fn day3a_solution(input_file_name: &str) {
    let input_file_contents = read_file(&input_file_name.to_string());
    let mut total_priority: isize = 0;
    for items in get_all_values(&input_file_contents) {
        let rucksack = Rucksack::put_items(&items);
        let common = rucksack.find_common_item();
        total_priority += isize::try_from(get_priority(&common)).unwrap();
    }
    println!("Total priority of items in the rucksacks: {total_priority}");
}

pub fn day3b_solution(input_file_contents: &str) {
    let input_file_contents = read_file(&input_file_contents.to_string());
    let rucksacks = get_all_values(&input_file_contents);
    let mut total_priority: isize = 0;
    let groups = Group::group_rucksacks(rucksacks);
    for group in groups {
        let common = group.find_common_item();
        total_priority += get_priority(&common);
    }
    println!("Total priority of groups: {total_priority}");
}
