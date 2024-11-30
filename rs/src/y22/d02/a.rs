use super::*;

#[aoc_proc::aoc_run(22-02a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input
        .as_ref()
        .lines()
        .map(|line| {
            let (opp, me) = line.ssplit_once(' ');
            let opp = P::ufrom(opp);
            let me = P::ufrom(me);
            me.points() + compete(opp, me).points()
        })
        .sum();
    Ok(sum)
}
