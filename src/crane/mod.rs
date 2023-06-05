#[derive(Debug)]
pub struct Move {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}

impl Move {
    pub fn crane_9000(stacks: &mut Vec<Vec<char>>, move_item: &Move) {
        for _ in 0..move_item.count {
            if let Some(moved) = stacks[move_item.from - 1].pop() {
                stacks[move_item.to - 1].push(moved);
            };
        }
    }

    pub fn crane_9001(stacks: &mut Vec<Vec<char>>, move_item: &Move) {
        let mut crates_moved: Vec<char> = vec![];
        for _ in 0..move_item.count {
            crates_moved.push(
                stacks[move_item.from - 1]
                    .pop()
                    .unwrap_or_else(|| panic!("Failed")),
            );
        }
        crates_moved.reverse();
        stacks[move_item.to - 1].extend_from_slice(&crates_moved)
    }

    fn parse_moves(move_item: &str) -> Move {
        let mut digits: Vec<usize> = vec![];
        move_item.split(' ').for_each(|item| {
            if item.chars().all(char::is_numeric) {
                let digit = item.parse::<usize>().unwrap();
                digits.push(digit);
            }
        });
        Move {
            count: digits[0] as usize,
            from: digits[1] as usize,
            to: digits[2] as usize,
        }
    }

    pub fn tops(stacks: &mut Vec<Vec<char>>) -> String {
        let mut tops = String::new();
        for stack in stacks {
            if let Some(top) = stack.pop() {
                tops += &top.to_string();
            }
        }
        tops
    }
}

pub fn parse_cargo(raw_input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let split_out: Vec<&str> = raw_input.split("\n\n").collect();
    let (stacks_raw, moves_raw) = (split_out[0], split_out[1]);
    let mut stacks: Vec<Vec<char>> = Vec::new();
    stacks_raw
        .split('\n')
        .rev()
        .enumerate()
        .for_each(|(index, layer)| {
            if index != 0 {
                layer
                    .chars()
                    .into_iter()
                    .skip(1)
                    .step_by(4)
                    .enumerate()
                    .for_each(|(i, c)| {
                        if index == 1 {
                            if c != ' ' {
                                stacks.push(vec![c]);
                            } else {
                                stacks.push(vec![]);
                            }
                        } else {
                            if c != ' ' {
                                stacks[i].push(c);
                            }
                        }
                    });
            }
        });

    let parsed_moves: Vec<Move> = moves_raw.split('\n').map(Move::parse_moves).collect();

    (stacks, parsed_moves)
}

#[cfg(test)]
mod test {
    use super::Move;

    #[test]
    fn test_act() {
        let moves = vec![Move {
            count: 3,
            from: 1,
            to: 2,
        }];
        let mut stacks = vec![vec!['A', 'B', 'C'], vec![]];
        for move_item in moves {
            Move::crane_9000(&mut stacks, &move_item);
        }
        let expected_stacks = vec![vec![], vec!['C', 'B', 'A']];
        assert_eq!(stacks, expected_stacks);
    }
}
