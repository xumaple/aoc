pub mod a;
pub mod b;

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y23/d04/sample.txt").unwrap()).unwrap(), 13);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y23/d04/input.txt").unwrap()).unwrap(), 23941);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y23/d04/sample.txt").unwrap()).unwrap(), 30);
    }

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y23/d04/input.txt").unwrap()).unwrap(),
            5571760
        );
    }
}
