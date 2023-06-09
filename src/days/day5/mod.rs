use crate::crane::{parse_cargo, Move};

pub fn day5a_solution(input_file_content: &str) {
    let (mut stacks, moves) = parse_cargo(input_file_content);
    moves
        .iter()
        .for_each(|move_item| Move::crane_9000(&mut stacks, move_item));
    let tops = Move::tops(&mut stacks);
    println!("Stack tops: {tops}");
}

pub fn day5b_solution(input_file_content: &str) {
    let (mut stacks, moves) = parse_cargo(input_file_content);
    moves
        .iter()
        .for_each(|move_item| Move::crane_9001(&mut stacks, move_item));
    let tops = Move::tops(&mut stacks);
    println!("Stack tops: {tops}");
}
