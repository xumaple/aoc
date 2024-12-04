use super::*;

#[aoc_proc::aoc_run(24-04b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let word_search = WordSearch::from_str(input.as_ref()).unwrap();
    Ok(word_search.search_x("MAS"))
}
