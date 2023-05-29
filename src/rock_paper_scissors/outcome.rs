#[derive(Debug)]
pub enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    pub fn match_outcome(string_outcome: &str) -> Outcome {
        match string_outcome {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _other => panic!("Unknown outcome string received {string_outcome}"),
        }
    }

    pub fn points(&self) -> isize {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
}
