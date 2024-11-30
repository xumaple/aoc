use super::*;

#[aoc_proc::aoc_run(22-02b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input
        .as_ref()
        .lines()
        .map(|line| {
            let (opp, round) = line.ssplit_once(' ');
            let opp = P::ufrom(opp);
            let round = Round::ufrom(round);
            round.points() + opp.compete(round).points()
        })
        .sum();
    Ok(sum)
}
