use super::*;

#[aoc_proc::aoc_run(22-06a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    match input
        .as_ref()
        .chars()
        .enumerate()
        .try_fold(Subroutine::new(4), |mut sub, (idx, char)| {
            match sub.add(char).is_marker() {
                true => Err(idx + 1),
                false => Ok(sub),
            }
        }) {
        Err(ans) => Ok(ans),
        Ok(_) => Err(E::ParseError)?,
    }
}
