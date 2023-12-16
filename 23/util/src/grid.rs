use crate::*;

#[derive(Clone)]
pub struct Grid<T>(Vec<Vec<T>>);

impl<T> FromStr for Grid<T>
where
    T: UnsafeFrom<char> + Clone,
{
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| line.chars().map(T::ufrom).collect_vec())
                .collect_vec(),
        ))
    }
}

impl<T> Grid<T> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn iter_rows<'a>(&'a self) -> std::slice::Iter<Vec<T>> {
        self.0.iter()
    }

    pub fn iter_rows_mut<'a>(&'a mut self) -> std::slice::IterMut<Vec<T>> {
        self.0.iter_mut()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter().flatten()
    }

    pub fn at(&mut self, coords: (usize, usize)) -> &mut T {
        &mut self.0[coords.0][coords.1]
    }
}

impl<T: Clone + Default + Copy> Grid<T> {
    pub fn invert(self) -> Self {
        let len = self.len();
        let width = self.width();
        Self(self.0.into_iter().enumerate().fold(
            vec![vec![T::default(); len]; width],
            |mut state, (j, v)| {
                v.iter().enumerate().for_each(|(i, x)| state[i][j] = *x);
                state
            },
        ))
    }

    pub fn rotate(self) -> Self {
        Self(
            self.iter_cols()
                .map(|v| v.into_iter().rev().collect_vec())
                .collect_vec(),
        )
    }

    pub fn iter_cols(&self) -> impl Iterator<Item = Vec<T>> {
        self.clone().invert().0.into_iter()
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().map(|line| format!("{:?}", line)).join("\n")
        )
    }
}
