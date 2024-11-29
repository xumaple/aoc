use util::*;

pub mod a;
pub mod b;

pub type IntType = u32;

pub fn hash(sequence: &str) -> u8 {
    sequence
        .chars()
        .fold(0 as IntType, |acc, c| {
            ((acc + c as u8 as IntType) * 17) % 256
        })
        .uinton()
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Sequence<'a>(&'a str);

impl<'a> Hash for Sequence<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u8(hash(self.0))
    }
}

#[derive(Clone, Debug)]
pub struct Box<'a>(Vec<(Sequence<'a>, IntType)>);

impl<'a> Box<'a> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn rm(&mut self, sequence: Sequence<'a>) {
        if let Some(pos) = self.0.iter().position(|tup| tup.0 == sequence) {
            let _ = self.0.remove(pos);
        }
    }

    pub fn add(&mut self, sequence: Sequence<'a>, num: IntType) {
        if let Some(pos) = self.0.iter().position(|tup| tup.0 == sequence) {
            self.0[pos].1 = num;
        } else {
            self.0.push((sequence, num));
        }
    }

    pub fn totals(&self) -> IntType {
        self.0
            .iter()
            .enumerate()
            .fold(0, |acc, (i, (_, num))| acc + (i as IntType + 1) * num)
    }
}

#[derive(Debug)]
pub struct Boxes<'a>(Vec<Box<'a>>);

impl<'a> Boxes<'a> {
    pub fn new() -> Self {
        Self(vec![Box::new(); 256])
    }

    pub fn add(&mut self, sequence: &'a str) {
        if sequence.chars().last().unwrap() == '-' {
            let label = &sequence[..sequence.len() - 1];
            let box_num = hash(label);
            self.0[box_num as usize].rm(Sequence::<'a>(label));
        } else {
            let (label, num) = sequence.ssplit_once('=');
            let box_num = hash(label);
            let num: IntType = num.uinto();
            self.0[box_num as usize].add(Sequence(label), num);
        }
    }

    pub fn sum(&self) -> IntType {
        self.0
            .iter()
            .enumerate()
            .map(|(box_num, b)| (1 + box_num) as IntType * b.totals())
            .sum()
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/d15/sample.txt").unwrap()).unwrap(), 1320);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/d15/input.txt").unwrap()).unwrap(), 503154);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/d15/sample.txt").unwrap()).unwrap(), 145);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/d15/input.txt").unwrap()).unwrap(), 251353);
    }
}
