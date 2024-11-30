use util::*;

pub mod a;
pub mod b;

pub type IntType = u64;

#[derive(Debug)]
pub struct Galaxy(IntType, IntType);

pub struct Universe {
    pub empty_rows: Vec<usize>,
    pub empty_cols: Vec<usize>,
    pub galaxies: Vec<Galaxy>,
}

impl FromStr for Universe {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut col_has_galaxy = vec![false; s.find('\n').unwrap()];
        let mut empty_rows = Vec::new();
        let mut galaxies = Vec::new();

        for (i, row) in s.lines().enumerate() {
            if let None = row.find('#') {
                empty_rows.push(i);
            }
            for (j, _) in row.match_indices('#') {
                col_has_galaxy[j] = true;
                galaxies.push(Galaxy(i as IntType, j as IntType));
            }
        }

        let empty_cols = col_has_galaxy
            .iter()
            .enumerate()
            .filter_map(|(i, has)| if *has { None } else { Some(i) })
            .collect_vec();

        Ok(Self {
            empty_rows,
            empty_cols,
            galaxies,
        })
    }
}

impl Universe {
    pub fn iter_rows_mut(&mut self) -> impl Iterator<Item = &mut IntType> {
        self.galaxies.iter_mut().map(|g| &mut g.0)
    }
    pub fn iter_cols_mut(&mut self) -> impl Iterator<Item = &mut IntType> {
        self.galaxies.iter_mut().map(|g| &mut g.1)
    }
    pub fn iter_rows(&mut self) -> impl Iterator<Item = &IntType> {
        self.galaxies.iter().map(|g| &g.0)
    }
    pub fn iter_cols(&mut self) -> impl Iterator<Item = &IntType> {
        self.galaxies.iter().map(|g| &g.1)
    }
    fn num_below(num: IntType, search: &Vec<usize>) -> IntType {
        match search.iter().position(|x| *x as IntType >= num) {
            Some(pos) => pos as IntType,
            None => search.len() as IntType,
        }
    }
    pub fn adjust_positions(&mut self, times_bigger: IntType) -> &mut Self {
        let empty_rows = self.empty_rows.clone();
        let empty_cols = self.empty_cols.clone();
        self.iter_rows_mut().for_each(|curr_row| {
            *curr_row += (times_bigger - 1) * Universe::num_below(*curr_row, &empty_rows)
        });
        self.iter_cols_mut().for_each(|curr_col| {
            *curr_col += (times_bigger - 1) * Universe::num_below(*curr_col, &empty_cols)
        });
        self
    }

    pub fn all_distances_between_galaxies(&mut self) -> IntType {
        let rows_total = {
            let mut sum = self.iter_rows().map(|x| *x).sum::<IntType>();
            let mut nums_left = self.galaxies.len() as IntType;
            self.iter_rows().fold(0, |val, x| {
                let ret = sum - *x * nums_left;
                nums_left -= 1;
                sum -= *x;
                val + ret
            })
        };
        let cols_total = {
            let mut sum = self.iter_cols().map(|x| *x).sum::<IntType>();
            let mut nums_left = self.galaxies.len() as IntType;
            self.iter_cols().into_iter().sorted().fold(0, |val, x| {
                let ret = sum - *x * nums_left;
                nums_left -= 1;
                sum -= *x;
                val + ret
            })
        };
        rows_total + cols_total
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y23/d11/sample.txt").unwrap()).unwrap(), 374);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y23/d11/input.txt").unwrap()).unwrap(), 9274989);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y23/d11/sample.txt").unwrap()).unwrap(), 82000210);
    }

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y23/d11/input.txt").unwrap()).unwrap(),
            357134560737
        );
    }
}
