use core::iter::Iterator;

use util::*;

pub mod a;
pub mod b;

#[derive(Debug, Clone)]
struct Crates {
    crates: Vec<VecDeque<char>>,
    instructions: Vec<Instruction>,
}

impl FromStr for Crates {
    type Err = E;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let mut it = s.lines();
        let crates = match it.try_fold(vec![], |mut acc, el| match el.len() {
            0 => Err(acc), // Use the error branch to early propagate here
            len => {
                if acc.is_empty() {
                    // Initialize
                    acc = vec![VecDeque::new(); len / 4 + 1];
                }
                for (acc_idx, el_idx) in (1..len).step_by(4).enumerate() {
                    let c = el[el_idx..el_idx + 1].chars().next().unwrap();
                    if c != ' ' {
                        acc[acc_idx].push_front(c);
                    }
                }
                Ok(acc)
            }
        }) {
            Err(acc) => acc,
            Ok(_) => return Err(E::ParseError), // try_fold returning ok means it didn't early exit
        };

        Ok(Self {
            crates,
            instructions: it
                .map(Instruction::from_str)
                .collect::<Result<Vec<_>, E>>()?,
        })
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    pub num_moves: usize,
    pub from: usize,
    pub to: usize,
}

impl FromStr for Instruction {
    type Err = E;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let (num_moves, s) = s[5..].ssplit_once(" from ");
        let (from, to) = s.ssplit_once(" to ");
        Ok(Self {
            num_moves: num_moves.uinto(),
            from: from.uinto(),
            to: to.uinto(),
        })
    }
}

impl Crates {
    pub fn execute(mut self, order: Order) -> Self {
        for instruction in &self.instructions {
            // for _ in 0..instruction.num_moves {
            //     let el = self.crates[instruction.from - 1].pop_back().unwrap();
            //     self.crates[instruction.to - 1].push_back(el);
            // }
            let split_at = self.crates[instruction.from - 1].len() - instruction.num_moves;
            let mut split = self.crates[instruction.from - 1].split_off(split_at);
            split = match order {
                Order::FILO => split.into_iter().rev().collect::<VecDeque<_>>(),
                Order::FIFO => split,
            };

            self.crates[instruction.to - 1].append(&mut split);
        }
        self
    }

    pub fn top(self) -> String {
        self.crates
            .into_iter()
            .map(|dq| dq.back().unwrap().clone())
            .join("")
    }
}

enum Order {
    FIFO,
    FILO,
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y22/d05/sample.txt").unwrap()).unwrap(), "CMZ");
    }

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y22/d05/input.txt").unwrap()).unwrap(),
            "ZRLJGSCTR"
        );
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y22/d05/sample.txt").unwrap()).unwrap(), "MCD");
    }

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y22/d05/input.txt").unwrap()).unwrap(),
            "PRTTGRFPB"
        );
    }
}
