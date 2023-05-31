#[derive(Debug)]
pub struct Member {
    pub min: isize,
    pub max: isize,
}

impl Member {
    pub fn new() -> Member {
        Member { min: 0, max: 0 }
    }

    pub fn set(&mut self, min: &str, max: &str) {
        self.min = min
            .parse::<isize>()
            .unwrap_or_else(|_| panic!("Failed to parse min {min}"));
        self.max = max
            .parse::<isize>()
            .unwrap_or_else(|_| panic!("Failed to parse max {max}"));
    }

    pub fn populate(&mut self, items: Vec<&str>) {
        if items.len() == 1 {
            self.set(items[0], items[0]);
        } else {
            self.set(items[0], items[1]);
        }
    }
}

impl Default for Member {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Pair(pub Member, pub Member);

impl Pair {
    pub fn new() -> Pair {
        Pair(Member::new(), Member::new())
    }

    pub fn build_pair(raw_members: Vec<&str>) -> Pair {
        let item1 = raw_members[0];
        let item1_split: Vec<&str> = item1.split('-').collect();
        let mut member1 = Member::new();
        member1.populate(item1_split);

        let item2 = raw_members[1];
        let item2_split: Vec<&str> = item2.split('-').collect();
        let mut member2 = Member::new();
        member2.populate(item2_split);

        Pair(member1, member2)
    }
}
impl Default for Pair {
    fn default() -> Self {
        Self::new()
    }
}
