use grid_vec::{CursorMut, Grid};
use util::*;

pub mod a;
pub mod b;

pub type IntType = usize;

#[derive(Clone, PartialEq, Eq, Copy)]
enum Space {
    Wall,
    Robot,
    Box,
    Empty,
    BoxLeft,
    BoxRight,
}

impl Debug for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Box => 'O',
                Self::Empty => '.',
                Self::Robot => '@',
                Self::Wall => '#',
                Self::BoxLeft => '[',
                Self::BoxRight => ']',
            }
        )
    }
}

impl UnsafeFrom<char> for Space {
    fn ufrom(input: char) -> Self {
        match input {
            '#' => Self::Wall,
            'O' => Self::Box,
            '@' => Self::Robot,
            '.' => Self::Empty,
            _ => panic!(),
        }
    }
}

impl Space {
    pub fn split(self) -> [Self; 2] {
        match self {
            Self::Box => [Self::BoxLeft, Self::BoxRight],
            Self::Empty => [Self::Empty, Self::Empty],
            Self::Robot => [Self::Robot, Self::Empty],
            Self::Wall => [Self::Wall, Self::Wall],
            Self::BoxLeft => unimplemented!(),
            Self::BoxRight => unimplemented!(),
        }
    }
}

struct Lake {
    grid: Grid<Space>,
    robot: Position,
}

impl FromStr for Lake {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = s.parse::<Grid<Space>>()?;
        let robot = grid
            .iter_flat_mut()
            .find(|cs| *cs.val() == Space::Robot)
            .unwrap()
            .index;
        Ok(Self { grid, robot })
    }
}

impl Lake {
    pub fn move_robot(self, s: &str) -> Self {
        s.lines().join("").chars().fold(self, |lake, mv| {
            lake.move_once(match mv {
                '<' => Direction::L,
                '^' => Direction::U,
                '>' => Direction::R,
                'v' => Direction::D,
                _ => panic!(),
            })
        })
    }

    pub fn turn_big(self) -> Self {
        let grid = Grid::new(
            self.grid
                .into_iter_raw()
                .map(|row| row.map(Space::split).flatten().collect_vec())
                .collect_vec(),
        );
        Self {
            grid,
            robot: Position::new(self.robot.x, self.robot.y * 2),
        }
    }

    fn robot_cursor(&mut self) -> CursorMut<Space> {
        self.grid.cursor_mut(self.robot)
    }

    fn move_once(mut self, dir: Direction) -> Self {
        let mut cs = self.robot_cursor();
        let mut next_cs = cs.next(dir).unwrap();
        if match *next_cs.val() {
            Space::Wall => false,
            Space::Robot => panic!(),
            Space::Box => Self::push(dir, next_cs),
            Space::Empty => true,
            Space::BoxLeft => match dir.is_horizontal() {
                true => Self::push(dir, next_cs),
                false => {
                    let can_push =
                        Self::can_push_big_box(dir, (next_cs, next_cs.next(Direction::R).unwrap()));
                    if can_push {
                        Self::push_big_box(dir, (next_cs, next_cs.next(Direction::R).unwrap()));
                    }
                    can_push
                }
            },
            Space::BoxRight => match dir.is_horizontal() {
                true => Self::push(dir, next_cs),
                false => {
                    let can_push =
                        Self::can_push_big_box(dir, (next_cs.next(Direction::L).unwrap(), next_cs));
                    if can_push {
                        Self::push_big_box(dir, (next_cs.next(Direction::L).unwrap(), next_cs));
                    }
                    can_push
                }
            },
        } {
            *next_cs.val_mut() = Space::Robot;
            *cs.val_mut() = Space::Empty;
            self.robot.step(dir).unwrap();
        }
        self
    }

    fn push(dir: Direction, mut cs: CursorMut<Space>) -> bool {
        let mut next_cs = cs.next(dir).unwrap();
        if match *next_cs.val() {
            Space::Wall => false,
            Space::Empty => true,
            Space::BoxLeft => Self::push(dir, next_cs),
            Space::BoxRight => Self::push(dir, next_cs),
            Space::Box => Self::push(dir, next_cs),
            Space::Robot => panic!(),
        } {
            std::mem::swap(cs.val_mut(), next_cs.val_mut());
            true
        } else {
            false
        }
    }

    fn can_push_big_box(dir: Direction, cs: (CursorMut<Space>, CursorMut<Space>)) -> bool {
        let (cs1, cs2) = cs;
        let cs1_next = cs1.next(dir).unwrap();
        let cs2_next = cs2.next(dir).unwrap();
        match (*cs1_next.val(), *cs2_next.val()) {
            (Space::Empty, Space::Empty) => true,
            (Space::BoxLeft, Space::BoxRight) => Self::can_push_big_box(dir, (cs1_next, cs2_next)),
            (Space::Empty, Space::BoxLeft) => {
                Self::can_push_big_box(dir, (cs2_next, cs2_next.next(Direction::R).unwrap()))
            }
            (Space::BoxRight, Space::Empty) => {
                Self::can_push_big_box(dir, (cs1_next.next(Direction::L).unwrap(), cs1_next))
            }
            (Space::BoxRight, Space::BoxLeft) => {
                Self::can_push_big_box(dir, (cs2_next, cs2_next.next(Direction::R).unwrap()))
                    && Self::can_push_big_box(dir, (cs1_next.next(Direction::L).unwrap(), cs1_next))
            }
            _ => false,
        }
    }

    fn push_big_box(dir: Direction, cs: (CursorMut<Space>, CursorMut<Space>)) {
        let (mut cs1, mut cs2) = cs;
        let mut cs1_next = cs1.next(dir).unwrap();
        let mut cs2_next = cs2.next(dir).unwrap();
        if match (*cs1_next.val(), *cs2_next.val()) {
            (Space::Empty, Space::Empty) => true,
            (Space::BoxLeft, Space::BoxRight) => {
                Self::push_big_box(dir, (cs1_next, cs2_next));
                true
            }
            (Space::Empty, Space::BoxLeft) => {
                Self::push_big_box(dir, (cs2_next, cs2_next.next(Direction::R).unwrap()));
                true
            }
            (Space::BoxRight, Space::Empty) => {
                Self::push_big_box(dir, (cs1_next.next(Direction::L).unwrap(), cs1_next));
                true
            }
            (Space::BoxRight, Space::BoxLeft) => {
                Self::push_big_box(dir, (cs1_next.next(Direction::L).unwrap(), cs1_next));
                Self::push_big_box(dir, (cs2_next, cs2_next.next(Direction::R).unwrap()));
                true
            }
            _ => false,
        } {
            std::mem::swap(cs1.val_mut(), cs1_next.val_mut());
            std::mem::swap(cs2.val_mut(), cs2_next.val_mut());
        }
    }

    pub fn gps_sum(&self) -> IntType {
        self.grid
            .iter_flat()
            .map(|cs| {
                if *cs.val() == Space::Box || *cs.val() == Space::BoxLeft {
                    cs.index.x * 100 + cs.index.y
                } else {
                    0
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d15/sample.txt").unwrap()).unwrap(), 2028);
    }

    #[test]
    fn sample_2() {
        assert_eq!(
            run(read("src/y24/d15/sample2.txt").unwrap()).unwrap(),
            10092
        );
    }

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y24/d15/input.txt").unwrap()).unwrap(),
            1412971
        );
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d15/sample2.txt").unwrap()).unwrap(), 9021);
    }

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y24/d15/input.txt").unwrap()).unwrap(),
            1429299
        );
    }
}
