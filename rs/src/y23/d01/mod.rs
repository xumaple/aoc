pub mod a;
pub mod b;

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y23/d01/sample-a.txt").unwrap()).unwrap(), 142);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y23/d01/input.txt").unwrap()).unwrap(), 54916);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y23/d01/sample-b.txt").unwrap()).unwrap(), 281);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y23/d01/input.txt").unwrap()).unwrap(), 54728);
    }
}
