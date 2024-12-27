use grid_vec::{CursorMut, Grid};
use util::*;

pub mod a;
pub mod b;

pub type IntType = usize;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Space {
    Wall,
    Path(Option<IntType>),
    Start,
    End,
}

impl Space {
    pub fn is_path(&self) -> bool {
        match *self {
            Self::Path(_) => true,
            _ => false,
        }
    }

    pub fn is_wall(&self) -> bool {
        match *self {
            Self::Wall => true,
            _ => false,
        }
    }

    pub fn get_path_score(&self) -> Option<IntType> {
        match *self {
            Self::Path(opt) => opt,
            _ => None,
        }
    }
}

impl UnsafeFrom<char> for Space {
    fn ufrom(input: char) -> Self {
        match input {
            '#' => Self::Wall,
            '.' => Self::Path(None),
            'S' => Self::Start,
            'E' => Self::End,
            _ => panic!(),
        }
    }
}

struct RaceTrack {
    grid: Grid<Space>,
    track: Vec<CursorMut<Space>>,
}

impl FromStr for RaceTrack {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            grid: s.parse()?,
            track: Vec::new(),
        })
    }
}

impl RaceTrack {
    pub fn traverse(&mut self) -> &mut Self {
        let mut curr = self
            .grid
            .iter_flat_mut()
            .find(|space| *space.val() == Space::Start)
            .unwrap();
        let end = self
            .grid
            .iter_flat()
            .find(|space| *space.val() == Space::End)
            .unwrap();

        let mut dir = Direction::iter_clockwise()
            .filter_map(|dir| {
                curr.next(dir)
                    .and_then(|cs| cs.val().is_path().then_some(dir))
            })
            .unext();
        let mut path_idx = 0;
        *curr.val_mut() = Space::Path(Some(path_idx));
        self.track.push(curr);
        loop {
            curr.step(dir).unwrap();
            path_idx += 1;
            *curr.val_mut() = Space::Path(Some(path_idx));
            self.track.push(curr);

            if curr.index == end.index {
                break;
            }

            dir = Direction::iter_clockwise()
                .filter(|next_dir| {
                    !next_dir.is_opposite(dir)
                        && curr.next(*next_dir).is_some_and(|cs| !cs.val().is_wall())
                })
                .unext();
        }

        self
    }

    fn find_cheats_for_cs(
        cs: &CursorMut<Space>,
        len: IntType,
    ) -> impl Iterator<Item = IntType> + use<'_> {
        let score = cs.val().get_path_score().unwrap();
        let ilen = len as isize;
        // println!("score {score} at {:?}", cs.index);
        (0..len as isize)
            .flat_map(move |i| {
                [
                    (-1 * ilen, 0, 1, 1),
                    (0, ilen, 1, -1),
                    (ilen, 0, -1, -1),
                    (0, -1 * ilen, -1, 1),
                ]
                .into_iter()
                .map(move |(xi, yi, dx, dy)| (xi + dx * i, yi + dy * i))
            })
            .filter_map(move |(x, y)| {
                // println!("{x} {y}");
                cs.move_pos(SignedPosition::new(x, y)).and_then(|new_cs| {
                    // println!(
                    //     "new_score {:?} at {:?}",
                    //     cs.val().get_path_score(),
                    //     cs.index
                    // );
                    let new_score = new_cs.val().get_path_score();
                    if let Some(new_score) = new_score {
                        if new_score >= score + len + 50 {
                            println!(
                                "{:?} to {:?}, saved {} picoseconds ({score} to {new_score})",
                                cs.index,
                                new_cs.index,
                                new_score - score - len
                            );
                        }
                    }
                    new_score.and_then(|new_score| {
                        (new_score > score + len).then(|| new_score - score - len)
                    })
                })
                // .and_then(|new_score| {
                //     (new_score > score + len).then(|| new_score - score - len)
                // })
            })
    }

    pub fn ways_to_save_time(
        &self,
        cheat_time: IntType,
    ) -> impl Iterator<Item = IntType> + use<'_> {
        self.track.iter().flat_map(move |cs| {
            (6..cheat_time + 1).flat_map(|len| Self::find_cheats_for_cs(cs, len))
        })
    }
}

pub fn run_race(
    input: impl AsRef<str>,
    cheat_time: IntType,
    picoseconds: IntType,
) -> Result<IntType, BoxError> {
    let sum = input
        .as_ref()
        .parse::<RaceTrack>()?
        .traverse()
        .ways_to_save_time(cheat_time)
        .filter(|t| *t >= picoseconds)
        .count();
    Ok(sum)
}

#[cfg(test)]
mod test_a {
    use super::{a::run, run_race};
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(
            run_race(read("src/y24/d20/sample.txt").unwrap(), 2, 2).unwrap(),
            44
        );
        assert_eq!(
            run_race(read("src/y24/d20/sample.txt").unwrap(), 2, 4).unwrap(),
            30
        );
        assert_eq!(
            run_race(read("src/y24/d20/sample.txt").unwrap(), 2, 64).unwrap(),
            1
        );
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y24/d20/input.txt").unwrap()).unwrap(), 1445);
    }
}

#[cfg(test)]
mod test_b {
    use super::{b::run, run_race};
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(
            run_race(read("src/y24/d20/sample.txt").unwrap(), 20, 50).unwrap(),
            32
        );
        assert_eq!(
            run_race(read("src/y24/d20/sample.txt").unwrap(), 20, 58).unwrap(),
            25
        );
        assert_eq!(
            run_race(read("src/y24/d20/sample.txt").unwrap(), 20, 76).unwrap(),
            3
        );
    }

    // #[test]
    // fn offical() {
    //     assert_eq!(run(read("src/y24/d20/input.txt").unwrap()).unwrap(), 0);
    // }
}
