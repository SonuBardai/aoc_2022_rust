use super::moves::Move;
use super::outcome::Outcome;
use super::players::Player;

pub fn calculate_game_points(opponent: Player, you: Player) -> isize {
    match Move::get_winner(opponent.player_move, you.player_move) {
        Outcome::Lose => you.move_points + Outcome::Lose.points(),
        Outcome::Win => you.move_points + Outcome::Win.points(),
        Outcome::Draw => you.move_points + Outcome::Draw.points(),
    }
}
