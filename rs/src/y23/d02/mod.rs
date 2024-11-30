pub mod a;
pub mod b;

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y23/d02/sample.txt").unwrap()).unwrap(), 8);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y23/d02/input.txt").unwrap()).unwrap(), 2476);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y23/d02/sample.txt").unwrap()).unwrap(), 2286);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y23/d02/input.txt").unwrap()).unwrap(), 54911);
    }
}
