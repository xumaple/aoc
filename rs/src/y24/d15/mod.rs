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

    fn robot_cursor(&mut self) -> CursorMut<Space> {
        self.grid.cursor_mut(self.robot)
    }

    fn move_once(mut self, dir: Direction) -> Self {
        let mut cs = self.robot_cursor();
        // println!("{:?}", self.grid);
        // println!("about to go {dir:?}");
        loop {
            // println!("{:?}", cs.index);
            match *cs.next(dir).unwrap().val() {
                Space::Wall => return self,
                Space::Robot => panic!(),
                Space::Box => {
                    cs.step(dir).unwrap();
                }
                Space::Empty => {
                    let mut robot_cs = self.robot_cursor();
                    let mut next_robot_cs = robot_cs.next(dir).unwrap();
                    *cs.step(dir).unwrap().val_mut() = *next_robot_cs.val();
                    *next_robot_cs.val_mut() = Space::Robot;
                    *robot_cs.val_mut() = Space::Empty;
                    self.robot.step(dir).unwrap();
                    return self;
                }
            }
        }
    }

    pub fn gps_sum(&self) -> IntType {
        self.grid
            .iter_flat()
            .map(|cs| {
                if let Space::Box = cs.val() {
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
        assert_eq!(run(read("src/y24/d15/sample.txt").unwrap()).unwrap(), 0);
    }

    // #[test]
    // fn offical() {
    //     assert_eq!(run(read("src/y24/d15/input.txt").unwrap()).unwrap(), 0);
    // }
}
