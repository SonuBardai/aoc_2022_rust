pub trait CommunicationSystem {
    fn get_packet_marker(&self, window_length: usize) -> usize;
    fn get_packet_marker_dequeue(&self, window_length: usize) -> usize;
}

impl CommunicationSystem for String {
    fn get_packet_marker(&self, window_length: usize) -> usize {
        let mut window: Vec<char> = vec![];
        for (index, item) in self.chars().enumerate() {
            if !window.contains(&item) {
                window.push(item);
                if window.len() == window_length {
                    println!("window: {window:?}");
                    return index + 1;
                }
            } else {
                let existing_item = window.iter().position(|&i| i == item).unwrap();
                window = window[existing_item + 1..window.len()].to_vec();
                window.push(item);
            }
        }
        0
    }

    fn get_packet_marker_dequeue(&self, window_length: usize) -> usize {
        let mut window = std::collections::VecDeque::<char>::new();
        for (index, item) in self.chars().enumerate() {
            while window.contains(&item) {
                window.pop_back();
            }
            window.push_front(item);
            if window.len() == window_length {
                return index + 1;
            }
        }
        0
    }
}

pub fn start_of_packet_marker(input_stream: String) -> i32 {
    input_stream.get_packet_marker_dequeue(4) as i32
}

pub fn start_of_message_marker(input_stream: String) -> i32 {
    input_stream.get_packet_marker_dequeue(14) as i32
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::{start_of_message_marker, start_of_packet_marker};

    #[test]
    fn test_start_of_packet_marker() {
        let test_cases = HashMap::from([
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ]);
        for (test_case, expected) in test_cases {
            println!("Testing string {test_case:?}");
            assert_eq!(start_of_packet_marker(test_case.to_string()), expected);
            println!("Passed: {test_case:?}");
        }
    }

    #[test]
    fn test_start_of_message_marker() {
        let test_cases = HashMap::from([
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ]);
        for (test_case, expected) in test_cases {
            println!("Testing string {test_case:?}");
            assert_eq!(start_of_message_marker(test_case.to_string()), expected);
            println!("Passed: {test_case:?}");
        }
    }
}
