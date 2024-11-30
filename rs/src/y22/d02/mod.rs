use util::*;

pub mod a;
pub mod b;

pub type IntType = u32;

#[derive(Clone, Debug)]
pub enum P {
    Rock,
    Paper,
    Scissors,
}

impl UnsafeFrom<&str> for P {
    fn ufrom(input: &str) -> Self {
        match input {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!(),
        }
    }
}

impl P {
    pub fn points(&self) -> IntType {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn compete(&self, result: Round) -> Self {
        match result {
            Round::Win => match self {
                Self::Paper => Self::Scissors,
                Self::Rock => Self::Paper,
                Self::Scissors => Self::Rock,
            },
            Round::Draw => self.clone(),
            Round::Lose => match self {
                Self::Paper => Self::Rock,
                Self::Rock => Self::Scissors,
                Self::Scissors => Self::Paper,
            },
        }
    }
}

#[derive(Clone, Debug)]
pub enum Round {
    Win,
    Lose,
    Draw,
}

impl UnsafeFrom<&str> for Round {
    fn ufrom(input: &str) -> Self {
        match input {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!(),
        }
    }
}

impl Round {
    pub fn points(&self) -> IntType {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}

pub fn compete(opp: P, me: P) -> Round {
    match opp {
        P::Rock => match me {
            P::Rock => Round::Draw,
            P::Paper => Round::Win,
            P::Scissors => Round::Lose,
        },
        P::Paper => match me {
            P::Rock => Round::Lose,
            P::Paper => Round::Draw,
            P::Scissors => Round::Win,
        },
        P::Scissors => match me {
            P::Rock => Round::Win,
            P::Paper => Round::Lose,
            P::Scissors => Round::Draw,
        },
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y22/d02/sample.txt").unwrap()).unwrap(), 15);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y22/d02/input.txt").unwrap()).unwrap(), 8890);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y22/d02/sample.txt").unwrap()).unwrap(), 12);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y22/d02/input.txt").unwrap()).unwrap(), 10238);
    }
}
