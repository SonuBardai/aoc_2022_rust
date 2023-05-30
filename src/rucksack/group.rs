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
        return (
            self.rucksack1.combined_compartments(),
            self.rucksack2.combined_compartments(),
            self.rucksack3.combined_compartments(),
        );
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
        if current_group.is_empty() {
            groups.push(current_group);
        }
        groups
    }

    pub fn find_common_item(&self) -> char {
        let (items1, items2, items3) = self.all_items();
        todo!("Find common items in group of rucksacks")
    }
}
