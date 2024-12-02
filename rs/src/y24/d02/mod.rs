use util::*;

pub mod a;
pub mod b;

pub type IntType = i32;

fn safe_report<'a, I: Iterator<Item = &'a IntType>>(iter: I) -> bool {
    match iter
        .tuple_windows()
        .map(|(a, b)| (abs_diff(*a, *b), a > b))
        .try_reduce(
            |(d1, g1), (d2, g2)| match d1 < 4 && d1 > 0 && d2 < 4 && d2 > 0 && g1 == g2 {
                true => Ok((d2, g2)),
                false => Err(E::AlgorithmError("")),
            },
        ) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn safe_report_with_tolerance<'a, I: Iterator<Item = &'a IntType> + Clone>(
    iter: I,
    len_hint: usize,
) -> bool {
    (0..len_hint).any(|skip| {
        safe_report(
            iter.clone()
                .enumerate()
                .filter_map(|(i, e)| if i != skip { Some(e) } else { None }),
        )
    })
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d02/sample.txt").unwrap()).unwrap(), 2);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y24/d02/input.txt").unwrap()).unwrap(), 224);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d02/sample.txt").unwrap()).unwrap(), 4);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y24/d02/input.txt").unwrap()).unwrap(), 293);
    }
}
