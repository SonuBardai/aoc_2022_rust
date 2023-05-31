use super::{moves::Move, outcome::Outcome};
use std::fmt::Error;

#[derive(Debug)]
pub struct Player {
    pub player_move: Move,
    pub move_points: isize,
}

impl Player {
    fn extract_player(string_move: &str) -> Result<Player, Error> {
        let player_move = if let Ok(player_move) = Move::match_move(string_move) {
            player_move
        } else {
            panic!("Failed to extract player for move {string_move}")
        };
        let player = Player {
            player_move,
            move_points: player_move.points(),
        };
        Ok(player)
    }

    pub fn extract_players(round: &str) -> Result<(Player, Player), Error> {
        let players = round.split(' ').collect::<Vec<&str>>();
        if let (Ok(opponent), Ok(you)) = (
            Player::extract_player(players[0]),
            Player::extract_player(players[1]),
        ) {
            Ok((opponent, you))
        } else {
            panic!("Failed to extract players for round {round}");
        }
    }

    pub fn extract_players_from_outcome(round: &str) -> Result<(Player, Player), Error> {
        let (opponent_move, outcome) =
            if let [opponent_move, outcome] = round.split(' ').collect::<Vec<&str>>()[..] {
                (opponent_move, outcome)
            } else {
                panic!("Invalid round received {round}. Cannot extract player and outcome.");
            };
        let opponent = Player::extract_player(opponent_move).unwrap_or_else(|_| panic!("Failed to extract player from move {opponent_move}"));
        let outcome = Outcome::match_outcome(outcome);
        let you_move = opponent.player_move.match_move_from_outcome(&outcome);
        let you = Player {
            player_move: you_move,
            move_points: you_move.points(),
        };
        // println!(
        //     "Opponent {:?} ({}) | You {:?} ({}) | Expected outcome {:?} ({}) | Total ({})",
        //     opponent.player_move,
        //     opponent.move_points,
        //     you.player_move,
        //     you.move_points,
        //     outcome,
        //     outcome.points(),
        //     you.move_points + outcome.points()
        // );
        Ok((opponent, you))
    }
}
