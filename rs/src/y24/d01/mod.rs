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
        assert_eq!(run(read("src/y24/d01/sample.txt").unwrap()).unwrap(), 11);
    }

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y24/d01/input.txt").unwrap()).unwrap(),
            1110981
        );
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d01/sample.txt").unwrap()).unwrap(), 31);
    }

    // #[test]
    // fn offical() {
    //     assert_eq!(run(read("src/y24/d01/input.txt").unwrap()).unwrap(), 0);
    // }
}
