pub mod a;
pub mod b;

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y23/d03/sample.txt").unwrap()).unwrap(), 4361);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y23/d03/input.txt").unwrap()).unwrap(), 530495);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(
            run(read("src/y23/d03/sample.txt").unwrap()).unwrap(),
            467835
        );
    }

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y23/d03/input.txt").unwrap()).unwrap(),
            80253814
        );
    }
}
