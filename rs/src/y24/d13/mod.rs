use util::*;

pub mod a;
pub mod b;

pub type IntType = i64;

#[derive(Copy, Clone, Debug)]
struct ClawMachine {
    b1: Button,
    b2: Button,
    prize: Prize,
}

#[derive(Copy, Clone, Debug)]
struct Button {
    cost: IntType,
    x: IntType,
    y: IntType,
    multiplier: IntType,
}

#[derive(Copy, Clone, Debug)]
struct Prize {
    x: IntType,
    y: IntType,
}

impl Add for Button {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            cost: self.cost() + rhs.cost(),
            x: self.x() + rhs.x(),
            y: self.y() + rhs.y(),
            multiplier: 1,
        }
    }
}

impl Mul<IntType> for Button {
    type Output = Self;
    fn mul(mut self, rhs: IntType) -> Self::Output {
        self.multiplier *= rhs;
        self
    }
}

impl PartialEq<Prize> for Button {
    fn eq(&self, other: &Prize) -> bool {
        self.x() == other.x && self.y() == other.y
    }
}

impl Button {
    pub fn from_iter<'a, I: Iterator<Item = &'a str>>(cost: IntType, input: &mut I) -> Self {
        Self {
            cost,
            x: input.next().unwrap().uinto(),
            y: input.next().unwrap().uinto(),
            multiplier: 1,
        }
    }

    fn x(&self) -> IntType {
        self.multiplier * self.x
    }

    fn y(&self) -> IntType {
        self.multiplier * self.y
    }

    pub fn cost(&self) -> IntType {
        self.multiplier * self.cost
    }
}

impl Prize {
    pub fn from_iter<'a, I: Iterator<Item = &'a str>>(offset: IntType, input: &mut I) -> Self {
        Self {
            x: IntType::ufrom(input.next().unwrap()) + offset,
            y: IntType::ufrom(input.next().unwrap()) + offset,
        }
    }
}

impl ClawMachine {
    pub fn from_iter<'a, I: Iterator<Item = &'a str>>(offset: IntType, mut input: I) -> Self {
        let _ = input.next();
        let b1 = Button::from_iter(3, &mut input);
        let b2 = Button::from_iter(1, &mut input);
        let prize = Prize::from_iter(offset, &mut input);
        Self { b1, b2, prize }
    }

    pub fn optimal_cost(&self) -> IntType {
        let b1 = &self.b1;
        let b2 = &self.b2;
        let p = &self.prize;

        let n_b2 = (p.x * b1.y - p.y * b1.x) / (b2.x * b1.y - b1.x * b2.y);
        let n_b1 = (p.y - n_b2 * b2.y) / b1.y;

        let b = *b1 * n_b1 + *b2 * n_b2;
        if b == *p {
            b.cost()
        } else {
            0
        }
    }
}
#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d13/sample.txt").unwrap()).unwrap(), 480);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y24/d13/input.txt").unwrap()).unwrap(), 27157);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(
            run(read("src/y24/d13/sample.txt").unwrap()).unwrap(),
            875318608908
        );
    }

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y24/d13/input.txt").unwrap()).unwrap(),
            104015411578548
        );
    }
}
