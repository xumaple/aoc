use super::*;

#[aoc_proc::aoc_run(24-14b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    Ok((0..10000)
        .into_iter()
        .try_fold(
            (input.as_ref().parse::<Library>()?, 0 as IntType),
            |(mut library, iterations), _| match library.has_christmas_tree() {
                true => {
                    println!("{}", library.display());
                    Err(iterations)
                }
                false => Ok((library.simulate(1), iterations + 1)),
            },
        )
        .unwrap_err())
}
