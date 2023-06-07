use crate::rock_paper_scissors::calculate_total_score;
use crate::rock_paper_scissors::players::Player;
use crate::utils::read_file;

pub fn day2a_solution(input_file_name: &str) {
    let read_lines = read_file(input_file_name);
    let total_score = calculate_total_score(&read_lines, Player::extract_players);
    println!("Treating Col-2 as Your moves. Total score: {total_score}");
}

pub fn day2b_solution(input_file_name: &str) {
    let read_lines = read_file(input_file_name);
    let total_score = calculate_total_score(&read_lines, Player::extract_players_from_outcome);
    println!("Treating Col-2 as expected outcomes. Total score: {total_score}");
}
