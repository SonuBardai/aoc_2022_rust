static LOWER_CASE_ASCII_START: u8 = 97;
static UPPER_CASE_ASCII_START: u8 = 65;

pub fn get_priority(common: &char) -> isize {
    let ascii_value = *common as u8;
    let priority = if common.is_ascii_lowercase() {
        ascii_value - LOWER_CASE_ASCII_START + 1
    } else {
        ascii_value - UPPER_CASE_ASCII_START + 1 + 26
    };
    isize::try_from(priority).unwrap()
}

#[cfg(test)]
mod test {
    use crate::rucksack::items::get_priority;

    #[test]
    fn test_get_priority() {
        let mut character = 'a';
        assert_eq!(get_priority(&character), 1);
        // println!("Test case passed {character}");

        character = 'A';
        println!("Priority of A {}", character as u8);
        assert_eq!(get_priority(&character), 27);
        // println!("Test case passed {character}");

        character = 'g';
        assert_eq!(get_priority(&character), 7);
        // println!("Test case passed {character}");

        character = 'Z';
        assert_eq!(get_priority(&character), 52);
        // println!("Test case passed {character}");
    }
}
