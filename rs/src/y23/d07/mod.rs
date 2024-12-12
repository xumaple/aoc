pub mod a;
pub mod b;

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y23/d07/sample.txt").unwrap()).unwrap(), 6440);
    }

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y23/d07/input.txt").unwrap()).unwrap(),
            248569531
        );
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y23/d07/sample.txt").unwrap()).unwrap(), 5905);
    }

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y23/d07/input.txt").unwrap()).unwrap(),
            250382098
        );
    }
}
