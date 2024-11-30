use super::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct TopThree(BinaryHeap<Reverse<IntType>>);

impl TopThree {
    pub fn new() -> Self {
        Self(BinaryHeap::new())
    }

    pub fn sum(&self) -> IntType {
        self.0.iter().map(|i| i.0).sum()
    }

    pub fn add(mut self, new: IntType) -> Self {
        self.0.push(Reverse(new));
        if self.0.len() > 3 {
            self.0.pop();
        }
        self
    }
}

#[aoc_proc::aoc_run(22-01b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input
        .as_ref()
        .lines()
        .fold((TopThree::new(), 0), |(top_three, curr_sum), el| {
            if el.len() == 0 {
                (top_three.add(curr_sum), 0)
            } else {
                (top_three, curr_sum + IntType::ufrom(el))
            }
        })
        .0
        .sum();
    Ok(sum)
}
