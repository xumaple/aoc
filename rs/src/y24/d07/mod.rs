use util::*;

pub mod a;
pub mod b;

pub type IntType = u64;

struct Calibration {
    total: IntType,
    vals: Vec<IntType>,
}

#[derive(Debug, Clone)]
enum Op {
    Add,
    Mult,
    Concat,
}

impl Op {
    pub fn calc(&self, a: IntType, b: IntType) -> IntType {
        match self {
            Op::Add => a + b,
            Op::Mult => a * b,
            Op::Concat => format!("{a}{b}").uinto(),
        }
    }
}

struct Values<I: Iterator<Item = Op> + Clone>(Vec<IntType>, I);
impl<I: Iterator<Item = Op> + Clone> Values<I> {
    pub fn new(ops: I) -> Self {
        Self(Vec::new(), ops)
    }
    pub fn push(self, next: IntType) -> Self {
        Self(
            match self.0.is_empty() {
                false => self
                    .0
                    .into_iter()
                    .map(|v| self.1.clone().map(move |op| op.calc(v, next)))
                    .flatten()
                    .collect(),
                true => vec![next],
            },
            self.1,
        )
    }
}

impl FromStr for Calibration {
    type Err = E;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let (total, vals) = s.ssplit_once(": ");
        Ok(Self {
            total: total.uinto(),
            vals: vals.split(" ").map(IntType::ufrom).collect(),
        })
    }
}

impl Calibration {
    pub fn total_if_can_equal(&self, ops: impl Iterator<Item = Op> + Clone) -> IntType {
        self.vals
            .iter()
            .fold(Values::new(ops), |values, val| values.push(*val))
            .0
            .into_iter()
            .find(|x| *x == self.total)
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d07/sample.txt").unwrap()).unwrap(), 3749);
    }

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y24/d07/input.txt").unwrap()).unwrap(),
            21572148763543
        );
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d07/sample.txt").unwrap()).unwrap(), 11387);
    }

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y24/d07/input.txt").unwrap()).unwrap(),
            581941094529163
        );
    }
}
