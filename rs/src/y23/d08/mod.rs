use util::*;

pub mod a;
pub mod b;

pub type IntType = u64;
pub enum Choice {
    Right,
    Left,
}

pub struct Directions(Vec<Choice>);

impl FromStr for Directions {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .map(|c| match c {
                    'R' => Choice::Right,
                    'L' => Choice::Left,
                    _ => unimplemented!(),
                })
                .collect_vec(),
        ))
    }
}

impl Directions {
    pub fn iter(&self) -> std::slice::Iter<Choice> {
        self.0.iter()
    }
}

#[derive(Copy, Clone)]
pub struct MapEntry {
    left: usize,
    right: usize,
}

impl MapEntry {
    pub fn new() -> Self {
        Self {
            left: usize::MAX,
            right: usize::MAX,
        }
    }
}

pub struct Map {
    map: Vec<MapEntry>,
    indices: HashMap<String, usize>,
    dest: HashSet<usize>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            map: Vec::new(),
            indices: HashMap::new(),
            dest: HashSet::new(),
        }
    }

    pub fn get_entry_index(&mut self, entry: &str) -> usize {
        *self.indices.entry(entry.to_string()).or_insert_with(|| {
            self.map.push(MapEntry::new());
            self.map.len() - 1
        })
    }

    pub fn add_mappings<'a, F, G>(
        &mut self,
        lines: impl Iterator<Item = &'a str>,
        start_pred: F,
        end_pred: G,
    ) -> Vec<usize>
    where
        F: Fn(&str) -> bool,
        G: Fn(&str) -> bool,
    {
        lines
            .map(|line| {
                let (a, bc) = line.ssplit_once(" = ");
                let (b, c) = bc[1..9].ssplit_once(", ");

                let a_entry_index = self.get_entry_index(a);
                let mut a_entry = self.map[a_entry_index];
                a_entry.left = self.get_entry_index(b);
                a_entry.right = self.get_entry_index(c);
                self.map[a_entry_index] = a_entry;
                if end_pred(a) {
                    let _ = self.dest.insert(a_entry_index);
                }
                (start_pred(a), a_entry_index)
            })
            .filter(|(pred, _)| *pred)
            .map(|(_, i)| i)
            .collect()
    }

    pub fn traverse_steps(&self, directions: &Directions, start: usize) -> IntType {
        let mut steps = 0;
        let mut curr_ind = start;
        let mut it = directions.iter();
        // let dest = *self.indices.get(dest_string).unwrap();

        let mut next = || -> &Choice {
            match it.next() {
                Some(v) => v,
                None => {
                    it = directions.iter();
                    it.next().unwrap()
                }
            }
        };

        while !self.dest.contains(&curr_ind) {
            let entry = self.map[curr_ind];
            curr_ind = match *next() {
                Choice::Left => entry.left,
                Choice::Right => entry.right,
            };
            steps += 1;
        }
        steps
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y23/d08/sample-a.txt").unwrap()).unwrap(), 6);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y23/d08/input.txt").unwrap()).unwrap(), 16897);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y23/d08/sample-b.txt").unwrap()).unwrap(), 6);
    }

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y23/d08/input.txt").unwrap()).unwrap(),
            16563603485021
        );
    }
}
