use util::*;

pub mod a;
pub mod b;

pub type IntType = u32;

fn mul(s: &str) -> IntType {
    println!("{s}");
    Regex::new(r"mul\([0-9]*,[0-9]*\)")
        .unwrap()
        .find_iter(s)
        .map(|s| {
            let (a, b) = s.as_str().after("mul(").before(")").ssplit_once(',');
            IntType::ufrom(a) * IntType::ufrom(b)
        })
        .sum()
}

fn mul_dos(s: &str) -> IntType {
    s.split("do()")
        .map(|s2| {
            mul(match s2.split("don't()").next() {
                Some(s3) => s3,
                None => s2,
            })
        })
        .sum()
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(
            run("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))").unwrap(),
            161
        );
    }

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y24/d03/input.txt").unwrap()).unwrap(),
            175015740
        );
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(
            run("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))")
                .unwrap(),
            48
        );
    }

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y24/d03/input.txt").unwrap()).unwrap(),
            112272912
        );
    }
}
