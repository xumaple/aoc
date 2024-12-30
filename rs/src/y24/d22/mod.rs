use util::*;

pub mod a;
pub mod b;

pub type IntType = i64;

static MAX: IntType = 16777216;

fn mix_and_prune(num: IntType, num2: IntType) -> IntType {
    (num ^ num2) % MAX
}

fn pseudorandom(num: IntType) -> IntType {
    let num = mix_and_prune(num, num << 6);
    let num = mix_and_prune(num, num >> 5);
    let num = mix_and_prune(num, num << 11);
    num
}

fn iterate(mut num: IntType, iterations: u16) -> IntType {
    for _ in 0..iterations {
        num = pseudorandom(num);
    }
    num
}

struct Sequence {
    a: IntType,
    b: IntType,
    c: IntType,
    d: IntType,
    val: IntType,
}

impl Hash for Sequence {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_key().hash(state)
    }
}

impl Sequence {
    pub fn get_key(&self) -> (IntType, IntType, IntType, IntType) {
        (self.a, self.b, self.c, self.d)
    }

    pub fn get_val(&self) -> IntType {
        self.val
    }
}

impl PartialEq for Sequence {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c && self.d == other.d
    }
}

impl Eq for Sequence {}

fn find_best_price_sequence<I: Iterator<Item = IntType>>(iter: I) -> IntType {
    iter.flat_map(|mut secret| {
        std::iter::repeat_with(move || {
            let tmp = secret;
            secret = pseudorandom(secret);
            tmp % 10
        })
        .take(2001)
        .tuple_windows()
        .map(|(a, b)| (b - a, b))
        .tuple_windows()
        .map(|((a, _), (b, _), (c, _), (d, val))| Sequence { a, b, c, d, val })
        .collect::<HashSet<Sequence>>()
        .into_iter()
    })
    .into_grouping_map_by(|seq| seq.get_key())
    .fold(0, |acc, _key, seq| acc + seq.get_val())
    .into_iter()
    .max_by_key(|(_, v)| *v)
    .unwrap()
    .1
}

#[cfg(test)]
mod test_a {
    use crate::y24::d22::pseudorandom;

    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(pseudorandom(123), 15887950);
        assert_eq!(
            run(read("src/y24/d22/sample.txt").unwrap()).unwrap(),
            37327623
        );
    }

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y24/d22/input.txt").unwrap()).unwrap(),
            18525593556
        );
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run("1\n2\n3\n2024").unwrap(), 23);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y24/d22/input.txt").unwrap()).unwrap(), 2089);
    }
}
