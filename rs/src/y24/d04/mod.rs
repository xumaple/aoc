use util::*;

pub mod a;
pub mod b;

pub type IntType = usize;

struct WordSearch(Grid<char>);

impl FromStr for WordSearch {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Grid::from_str(s)?))
    }
}

impl WordSearch {
    pub fn search(&self, p: &str) -> usize {
        let rows: usize = self
            .0
            .iter_rows()
            .map(|it| Self::str_counts(it.iter().collect::<String>(), p))
            .sum();
        let cols: usize = self
            .0
            .iter_cols()
            .map(|it| Self::str_counts(it.iter().collect::<String>(), p))
            .sum();
        let diags1: usize = self
            .0
            .iter_diags_positive()
            .map(|it| Self::str_counts(it.iter().collect::<String>(), p))
            .sum();
        let rotated = self.0.clone().rotate();
        let diags2: usize = rotated
            .iter_diags_positive()
            .map(|it| Self::str_counts(it.iter().collect::<String>(), p))
            .sum();
        rows + cols + diags1 + diags2
    }

    fn str_counts(s: String, p: &str) -> usize {
        s.matches(p).count() + s.rev().matches(p).count()
    }

    pub fn search_x(self, p: &str) -> usize {
        let grid = self.0.into_enumerated();
        let diags1 = grid
            .iter_diags_positive()
            .map(|col| Self::str_matches(col, p));
        let grid = grid.clone().rotate();
        let diags2 = grid
            .iter_diags_positive()
            .map(|col| Self::str_matches(col, p));

        diags1
            .flatten()
            .collect::<HashSet<Position>>()
            .intersection(
                &diags2
                    .flatten()
                    .map(|pos| Position::new(pos.x, pos.y - 2))
                    .collect::<HashSet<Position>>(),
            )
            .count()
    }

    fn str_matches(v: Vec<PositionT<char>>, p: &str) -> impl Iterator<Item = Position> {
        let (positions, s): (Vec<Position>, String) = v.into_iter().unzip();
        let indices = s
            .indices(p)
            .chain(s.indices(&p.rev()))
            .collect::<HashSet<usize>>();
        positions
            .into_iter()
            .enumerate()
            .filter_map(move |(idx, pos)| match indices.contains(&idx) {
                true => Some(pos),
                false => None,
            })
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d04/sample.txt").unwrap()).unwrap(), 18);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y24/d04/input.txt").unwrap()).unwrap(), 2573);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d04/sample.txt").unwrap()).unwrap(), 9);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y24/d04/input.txt").unwrap()).unwrap(), 1850);
    }
}
