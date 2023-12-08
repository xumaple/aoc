pub mod a;
pub mod b;

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/d06/sample.txt").unwrap()).unwrap(), 288);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/d06/input.txt").unwrap()).unwrap(), 1731600);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/d06/sample.txt").unwrap()).unwrap(), 71503);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/d06/input.txt").unwrap()).unwrap(), 40087680);
    }
}
