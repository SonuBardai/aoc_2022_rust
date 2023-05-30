pub mod group;
pub mod items;

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

    pub fn find_common_items_optimized(&self) -> char {
        ' '
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
}
