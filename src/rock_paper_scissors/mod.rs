pub mod game;
pub mod moves;
pub mod outcome;
pub mod players;
use players::Player;

use self::game::calculate_game_points;
use crate::utils::get_all_values;
use std::fmt::Error;

pub fn calculate_total_score(
    rounds: &str,
    extract_from_col2: fn(&str) -> Result<(Player, Player), Error>,
) -> isize {
    let mut you_points: isize = 0;
    for round in get_all_values(rounds) {
        if let Ok((opponent, you)) = extract_from_col2(round) {
            you_points += calculate_game_points(opponent, you);
        };
    }
    you_points
}
