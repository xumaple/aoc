pub mod a;
pub mod b;

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/d05/sample.txt").unwrap()).unwrap(), 35);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/d05/input.txt").unwrap()).unwrap(), 621354867);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/d05/sample.txt").unwrap()).unwrap(), 46);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/d05/input.txt").unwrap()).unwrap(), 15880236);
    }
}
