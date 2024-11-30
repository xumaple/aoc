use util::*;

pub mod a;
pub mod b;

pub type IntType = u32;

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y22/d01/sample.txt").unwrap()).unwrap(), 24000);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y22/d01/input.txt").unwrap()).unwrap(), 73211);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y22/d01/sample.txt").unwrap()).unwrap(), 45000);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y22/d01/input.txt").unwrap()).unwrap(), 213958);
    }
}
