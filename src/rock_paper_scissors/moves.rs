use std::fmt::Error;

use super::outcome::Outcome;

#[derive(Copy, Clone, Debug)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn points(&self) -> isize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    pub fn match_move(string_move: &str) -> Result<Move, Error> {
        match string_move {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            other => {
                panic!("Unknown symbol received {other}")
            }
        }
    }

    pub fn match_move_from_outcome(&self, expected_outcome: &Outcome) -> Move {
        match (self, expected_outcome) {
            (Move::Rock, Outcome::Lose) => Move::Scissors,
            (Move::Rock, Outcome::Win) => Move::Paper,
            (Move::Paper, Outcome::Lose) => Move::Rock,
            (Move::Paper, Outcome::Win) => Move::Scissors,
            (Move::Scissors, Outcome::Lose) => Move::Paper,
            (Move::Scissors, Outcome::Win) => Move::Rock,
            (_other, Outcome::Draw) => *self,
        }
    }

    pub fn get_winner(move_1: Move, move_2: Move) -> Outcome {
        match (move_1, move_2) {
            (Move::Rock, Move::Paper) => Outcome::Win,
            (Move::Paper, Move::Rock) => Outcome::Lose,
            (Move::Paper, Move::Scissors) => Outcome::Win,
            (Move::Scissors, Move::Paper) => Outcome::Lose,
            (Move::Scissors, Move::Rock) => Outcome::Win,
            (Move::Rock, Move::Scissors) => Outcome::Lose,
            _other => Outcome::Draw,
        }
    }
}
