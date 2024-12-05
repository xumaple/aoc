use util::*;

pub mod a;
pub mod b;

pub type IntType = u32;

struct Manual {
    rules: MultiMap<IntType, IntType>,
    updates: Vec<Update>,
}

struct Update(Vec<IntType>);

impl FromStr for Manual {
    type Err = E;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let mut rules = MultiMap::new();
        let mut it = s.lines();
        loop {
            let next = it.next().unwrap();
            if next.is_empty() {
                break;
            }
            let (a, b) = next.ssplit_once("|");
            rules.insert(a.uinto(), b.uinto());
        }
        let updates = it
            .map(|line| Update(line.split(',').map(IntType::ufrom).collect_vec()))
            .collect_vec();

        Ok(Self { rules, updates })
    }
}

impl Update {
    pub fn middle(&self) -> IntType {
        self.0[self.0.len() / 2]
    }

    pub fn is_right_order(&self, rules: &mut MultiMap<IntType, IntType>) -> bool {
        match self
            .0
            .iter()
            .try_fold(
                HashSet::<IntType>::new(),
                |mut prev_vals, &val| match prev_vals
                    .intersection(
                        &rules
                            .get_vals(val)
                            .map(Clone::clone)
                            .collect::<HashSet<IntType>>(),
                    )
                    .count()
                {
                    0 => {
                        prev_vals.insert(val);
                        Ok(prev_vals)
                    }
                    _ => Err(()),
                },
            ) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn into_right_order(mut self, rules: &mut MultiMap<IntType, IntType>) -> Self {
        // bubble sort
        for i in 0usize..self.0.len() - 1 {
            for j in 0usize..self.0.len() - 1 - i {
                if rules.contains(self.0[j + 1], self.0[j]) {
                    self.0.swap(j, j + 1);
                }
            }
        }
        self
    }
}

impl Manual {
    pub fn is_right_orders(mut self) -> IntType {
        self.updates
            .into_iter()
            .map(|update| match update.is_right_order(&mut self.rules) {
                true => update.middle(),
                false => 0,
            })
            .sum()
    }

    pub fn was_wrong_orders(mut self) -> IntType {
        self.updates
            .into_iter()
            .map(|update| match update.is_right_order(&mut self.rules) {
                true => 0,
                false => update.into_right_order(&mut self.rules).middle(),
            })
            .sum()
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d05/sample.txt").unwrap()).unwrap(), 143);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y24/d05/input.txt").unwrap()).unwrap(), 4814);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d05/sample.txt").unwrap()).unwrap(), 123);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y24/d05/input.txt").unwrap()).unwrap(), 5448);
    }
}
