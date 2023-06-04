pub fn start_of_packet_marker(input_stream: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::start_of_packet_marker;

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
            assert_eq!(start_of_packet_marker(test_case), expected);
        }
    }
}
