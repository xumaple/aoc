use aoc_proc::aoc_run;
use std::ops::Deref;
use util::*;

#[aoc_run(01b)]
pub fn run(input: impl AsRef<str>) -> Result<i32, BoxError> {
    let mappings = vec![
        ("1".as_bytes(), 1),
        ("2".as_bytes(), 2),
        ("3".as_bytes(), 3),
        ("4".as_bytes(), 4),
        ("5".as_bytes(), 5),
        ("6".as_bytes(), 6),
        ("7".as_bytes(), 7),
        ("8".as_bytes(), 8),
        ("9".as_bytes(), 9),
        ("one".as_bytes(), 1),
        ("two".as_bytes(), 2),
        ("three".as_bytes(), 3),
        ("four".as_bytes(), 4),
        ("five".as_bytes(), 5),
        ("six".as_bytes(), 6),
        ("seven".as_bytes(), 7),
        ("eight".as_bytes(), 8),
        ("nine".as_bytes(), 9),
    ];
    let rmappings = vec![
        ("1".as_bytes(), 1),
        ("2".as_bytes(), 2),
        ("3".as_bytes(), 3),
        ("4".as_bytes(), 4),
        ("5".as_bytes(), 5),
        ("6".as_bytes(), 6),
        ("7".as_bytes(), 7),
        ("8".as_bytes(), 8),
        ("9".as_bytes(), 9),
        ("eno".as_bytes(), 1),
        ("owt".as_bytes(), 2),
        ("eerht".as_bytes(), 3),
        ("ruof".as_bytes(), 4),
        ("evif".as_bytes(), 5),
        ("xis".as_bytes(), 6),
        ("neves".as_bytes(), 7),
        ("thgie".as_bytes(), 8),
        ("enin".as_bytes(), 9),
    ];
    let mut sum = 0;
    for l in input.as_ref().lines() {
        let i1 = find_multiple_inputs(l.as_bytes().iter(), &mappings);
        let i2 = find_multiple_inputs(l.as_bytes().iter().rev(), &rmappings);
        sum += 10 * i1 + i2;
    }

    Ok(sum)
}

fn find_multiple_inputs<T>(input_iter: T, mappings: &Vec<(&[u8], i32)>) -> i32
where
    T: Iterator,
    <T as Iterator>::Item: Deref,
    <<T as Iterator>::Item as Deref>::Target: PartialEq<u8>,
{
    let mut to_check = vec![0; mappings.len()];
    for c in input_iter {
        for ((m_i, val), tc_i) in std::iter::zip(mappings.iter(), to_check.iter_mut()) {
            if *c == m_i[*tc_i] {
                *tc_i += 1;
                if *tc_i == m_i.len() {
                    return *val;
                }
            } else {
                *tc_i = if *c == m_i[0] { 1 } else { 0 }
            }
        }
    }

    panic!("Didn't find anything...");
}
