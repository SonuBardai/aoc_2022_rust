pub mod group;
pub mod items;

use itertools::sorted;

#[derive(Debug)]
pub struct Rucksack {
    pub compartment1: String,
    pub compartment2: String,
}

impl Rucksack {
    pub fn new(items: Option<String>) -> Rucksack {
        match items {
            Some(items_string) => Rucksack::put_items(&items_string),
            None => Rucksack {
                compartment1: String::new(),
                compartment2: String::new(),
            },
        }
    }

    pub fn from(items: String) -> Rucksack {
        Rucksack::put_items(&items)
    }

    pub fn is_empty(&self) -> bool {
        self.compartment1.is_empty() && self.compartment2.is_empty()
    }

    pub fn combined_compartments(&self) -> String {
        self.compartment1.clone() + &self.compartment2
    }

    pub fn put_items(raw_items: &str) -> Rucksack {
        Rucksack {
            compartment1: raw_items[0..(raw_items.len() / 2)].to_string(),
            compartment2: raw_items[(raw_items.len() / 2)..raw_items.len()].to_string(),
        }
    }

    pub fn find_common_item(&self) -> char {
        // This can be optimized. Current solution is O(n^2)
        //     - Sort self.compartment2
        //     - for c1_char in compartment1
        //          binary search for c1_char in self.compartment2
        //          when found, break
        //     - This could make the complexity O(n logn)
        let mut common = ' ';
        for c2_char in self.compartment2.chars() {
            for c1_char in self.compartment1.chars() {
                if c2_char == c1_char {
                    common = c1_char;
                    break;
                }
            }
        }
        common
    }

    pub fn recursive_search(compartment: &str, item: &char) -> bool {
        if compartment.is_empty() {
            return false;
        } else if compartment.len() == 1 {
            return compartment.as_bytes()[0] == *item as u8;
        }
        let mid = compartment.len() / 2;
        let char_at_mid = compartment.as_bytes()[mid] as char;
        if char_at_mid == *item {
            true
        } else if (char_at_mid as u32) > (*item as u32) {
            return Rucksack::recursive_search(&compartment[0..mid], item);
        } else {
            return Rucksack::recursive_search(&compartment[mid + 1..compartment.len()], item);
        }
    }

    pub fn bin_search(compartment: &str, item: &char) -> bool {
        let sorted_compartment = sorted(compartment.chars()).collect::<String>();
        Rucksack::recursive_search(&sorted_compartment, item)
    }

    pub fn find_common_items_optimized(&self) -> char {
        let mut common: char = ' ';
        for c1_char in self.compartment1.chars() {
            if Rucksack::bin_search(&self.compartment2, &c1_char) {
                common = c1_char;
                break;
            }
        }
        common
    }
}

#[cfg(test)]
mod tests {
    use super::Rucksack;

    #[test]
    fn test_put_items() {
        let test_rucksack = Rucksack::put_items("abCDefGh");
        assert_eq!(test_rucksack.compartment1, "abCD");
        assert_eq!(test_rucksack.compartment2, "efGh");
    }

    #[test]
    fn test_find_common_item() {
        let test_rucksack = Rucksack::put_items("abCeDfGe");
        assert_eq!(test_rucksack.find_common_item(), 'e');
    }

    #[test]
    fn test_bin_search() {
        let test_rucksack = Rucksack::put_items("agAdlMjpfP");
        assert!(Rucksack::bin_search(&test_rucksack.compartment1, &'A'));
        assert!(!Rucksack::bin_search(&test_rucksack.compartment2, &'A'));
        let test_rucksack =
            Rucksack::put_items("CgmewIYIMTEwRJoTwPKAgAEjvxUjZrPhpkpWqvQNuwnEtySETxtuSfphuulj");
        assert!(Rucksack::bin_search(&test_rucksack.compartment1, &'g'));
        assert!(!Rucksack::bin_search(&test_rucksack.compartment1, &'Q'));
        assert!(Rucksack::bin_search(&test_rucksack.compartment1, &'U'));
        assert!(!Rucksack::bin_search(&test_rucksack.compartment2, &'a'));
        assert!(Rucksack::bin_search(&test_rucksack.compartment2, &'h'));
    }
}
