use super::items::get_priority;
use super::Rucksack;

#[derive(Debug)]
pub struct Group {
    pub rucksack1: Rucksack,
    pub rucksack2: Rucksack,
    pub rucksack3: Rucksack,
}

impl Group {
    pub fn new() -> Group {
        Group {
            rucksack1: Rucksack::new(None),
            rucksack2: Rucksack::new(None),
            rucksack3: Rucksack::new(None),
        }
    }

    pub fn from(rucksack1: String, rucksack2: String, rucksack3: String) -> Group {
        Group {
            rucksack1: Rucksack::new(Some(rucksack1)),
            rucksack2: Rucksack::new(Some(rucksack2)),
            rucksack3: Rucksack::new(Some(rucksack3)),
        }
    }

    pub fn push(&mut self, rucksack: Rucksack) {
        if self.rucksack1.is_empty() {
            self.rucksack1 = rucksack
        } else if self.rucksack2.is_empty() {
            self.rucksack2 = rucksack
        } else if self.rucksack3.is_empty() {
            self.rucksack3 = rucksack
        }
    }

    pub fn is_empty(&self) -> bool {
        self.rucksack1.is_empty() && self.rucksack2.is_empty() && self.rucksack3.is_empty()
    }

    pub fn all_items(&self) -> (String, String, String) {
        (
            self.rucksack1.combined_compartments(),
            self.rucksack2.combined_compartments(),
            self.rucksack3.combined_compartments(),
        )
    }

    pub fn group_rucksacks(rucksacks: Vec<&str>) -> Vec<Group> {
        let mut groups: Vec<Group> = vec![];
        let mut current_group: Group = Group::new();
        for (index, rucksack) in rucksacks.iter().enumerate() {
            if index % 3 == 0 && index != 0 {
                groups.push(current_group);
                current_group = Group::new();
                current_group.rucksack1 = Rucksack::new(Some(rucksack.to_string()));
            } else {
                current_group.push(Rucksack::new(Some(rucksack.to_string())));
            }
        }
        if !current_group.is_empty() {
            groups.push(current_group);
        }
        groups
    }

    pub fn find_common_items(&self) -> Vec<char> {
        let (rucksack1, rucksack2, rucksack3) = self.all_items();
        let mut common: Vec<char> = Vec::new();
        for r1_char in rucksack1.chars() {
            if Rucksack::bin_search(&rucksack2, &r1_char) && !common.contains(&r1_char) {
                common.push(r1_char);
            }
        }
        let mut final_common: Vec<char> = Vec::new();
        for common_char in common.iter() {
            if Rucksack::bin_search(&rucksack3, common_char) && !final_common.contains(common_char)
            {
                final_common.push(*common_char);
            }
        }
        final_common
    }

    pub fn get_characters_priority(common_characters: &Vec<char>) -> isize {
        let mut priority = 0;
        for common in common_characters {
            priority += get_priority(common);
        }
        priority
    }
}

impl Default for Group {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::Group;

    #[test]
    fn test_group_rucksacks() {
        let test_string = "a\nb\nc".split('\n').collect_vec();
        let groups = Group::group_rucksacks(test_string);
        assert_eq!(groups.len(), 1);

        let test_string = "a\nb\nc\nd\ne\nf".split('\n').collect_vec();
        let groups = Group::group_rucksacks(test_string);
        assert_eq!(groups.len(), 2);

        let test_string = "a\nb\nc\nd\ne\nf\ng\nh\ni".split('\n').collect_vec();
        let groups = Group::group_rucksacks(test_string);
        assert_eq!(groups.len(), 3);
    }

    #[test]
    fn test_find_common_item() {
        let test_group = Group::from(
            "rucksack1".to_string(),
            "rucksack2".to_string(),
            "rucksack3".to_string(),
        );
        let common = test_group.find_common_items();
        let expect_common = vec!['r', 'u', 'c', 'k', 's', 'a'];
        assert_eq!(common, expect_common);
    }
}
