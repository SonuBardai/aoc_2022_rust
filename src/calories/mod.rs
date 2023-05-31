mod utils;
pub use utils::*;

pub fn calculate_max_portion(portions: Vec<Vec<isize>>) -> isize {
    let mut max_calories = 0;
    for portion in portions {
        let cur_calories = portion.iter().sum();
        if cur_calories > max_calories {
            max_calories = cur_calories;
        }
    }
    max_calories
}

pub fn calculate_max_n_portions(portions: Vec<Vec<isize>>, n: usize) -> Vec<isize> {
    let mut calories_vec: Vec<isize> = Vec::new();
    for portion in portions {
        calories_vec.push(portion.iter().sum())
    }
    calories_vec.sort();
    let max_n: Vec<isize> = calories_vec.into_iter().rev().take(n).collect();
    max_n
}
