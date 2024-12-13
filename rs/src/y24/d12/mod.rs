use grid_map::{CursorMut, Grid};
use util::*;

pub mod a;
pub mod b;

pub type IntType = u32;

#[derive(Debug, Clone)]
struct Plot {
    id: char,
    accounted: bool,
}

impl UnsafeFrom<char> for Plot {
    fn ufrom(input: char) -> Self {
        Self {
            id: input,
            accounted: false,
        }
    }
}

#[derive(Debug, Clone)]
struct Section {
    perimeter: IntType,
    area: IntType,
}

impl Section {
    pub fn new() -> Self {
        Self {
            perimeter: 0,
            area: 0,
        }
    }

    pub fn add(mut self, perimeter: IntType) -> Self {
        self.perimeter += perimeter;
        self.area += 1;
        self
    }

    pub fn cost(&self) -> IntType {
        self.area * self.perimeter
    }
}

trait CheckSidesCalculation {
    fn check_sides_calculation(cs: CursorMut<Plot>) -> IntType;

    fn matches(cs1: CursorMut<Plot>, cs2: CursorMut<Plot>) -> bool {
        cs1.val().id == cs2.val().id
    }
}

struct Perimeter;
impl CheckSidesCalculation for Perimeter {
    fn check_sides_calculation(cs: CursorMut<Plot>) -> IntType {
        cs.check_sides(|plot| match plot {
            Some(plot) => {
                if plot.val().id == cs.val().id {
                    0
                } else {
                    1
                }
            }
            None => 1,
        })
        .sum()
    }
}

struct Sides;
impl CheckSidesCalculation for Sides {
    fn check_sides_calculation(cs: CursorMut<Plot>) -> IntType {
        let empty_in_dirs = Direction::iter_clockwise()
            .filter_map(|dir| {
                cs.next(dir)
                    .is_none_or(|new_cs| !Self::matches(cs, new_cs))
                    .then_some(dir)
            })
            .collect_vec();
        empty_in_dirs
            .iter()
            .map(
                |dir| match cs.exists_and_matches(dir.turn_L(), Self::matches) {
                    true => {
                        if cs
                            .next(dir.turn_L())
                            .unwrap()
                            .exists_and_matches(*dir, Self::matches)
                        {
                            1
                        } else {
                            0
                        }
                    }
                    false => 1,
                },
            )
            .sum()
    }
}

#[derive(Debug)]
struct Garden(Grid<Plot>);

impl FromStr for Garden {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Grid::from_str(s)?))
    }
}

impl Garden {
    fn get_entire_section<C: CheckSidesCalculation>(mut cs: CursorMut<Plot>) -> Section {
        cs.val_mut().accounted = true;
        let mut section = Section::new();
        let mut q = VecDeque::from([cs]);
        while let Some(cs) = q.pop_back() {
            q.extend(
                cs.check_sides(|plot| {
                    plot.map(|mut plot| {
                        if !plot.val().accounted && C::matches(plot.clone(), cs.clone()) {
                            plot.val_mut().accounted = true;
                            Some(plot)
                        } else {
                            None
                        }
                    })
                })
                .filter_map(Option::flatten),
            );
            section = section.add(C::check_sides_calculation(cs.clone()));
        }
        section
    }
    pub fn find_sections<C: CheckSidesCalculation>(
        &mut self,
    ) -> impl Iterator<Item = Section> + use<'_, C> {
        self.0.iter_flat_mut().filter_map(move |plot| {
            (!plot.val().accounted).then(|| Self::get_entire_section::<C>(plot))
        })
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d12/sample.txt").unwrap()).unwrap(), 1930);
    }

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y24/d12/input.txt").unwrap()).unwrap(),
            1549354
        );
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d12/sample.txt").unwrap()).unwrap(), 1206);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y24/d12/input.txt").unwrap()).unwrap(), 937032);
    }
}
