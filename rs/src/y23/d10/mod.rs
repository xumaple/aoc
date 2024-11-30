use util::*;

pub mod a;
pub mod b;

pub type IntType = u32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    Left,
    Down,
    Right,
    Up,
}
#[derive(Clone, Debug)]
pub struct Position(usize, usize);

impl Position {
    pub fn next(&mut self, c: char, prev_dir: Direction) -> Direction {
        let next_dir = match c {
            '|' => prev_dir,
            '-' => prev_dir,
            'L' => match prev_dir {
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                _ => panic!("{:?} {c} {:?}", self, prev_dir),
            },
            'J' => match prev_dir {
                Direction::Down => Direction::Left,
                Direction::Right => Direction::Up,
                _ => panic!("{:?} {c} {:?}", self, prev_dir),
            },
            '7' => match prev_dir {
                Direction::Right => Direction::Down,
                Direction::Up => Direction::Left,
                _ => panic!("{:?} {c} {:?}", self, prev_dir),
            },
            'F' => match prev_dir {
                Direction::Left => Direction::Down,
                Direction::Up => Direction::Right,
                _ => panic!("{:?} {c} {:?}", self, prev_dir),
            },
            _ => panic!("{:?} {c} {:?}", self, prev_dir),
        };
        self.move_space(&next_dir);
        next_dir
    }

    pub fn move_space(&mut self, dir: &Direction) {
        match dir {
            Direction::Down => self.0 += 1,
            Direction::Left => self.1 -= 1,
            Direction::Up => self.0 -= 1,
            Direction::Right => self.1 += 1,
        };
    }
}

#[derive(Debug)]
pub struct Map(Vec<Vec<char>>, Position);

impl FromStr for Map {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = Position(0, 0);
        Ok(Self(
            s.lines()
                .enumerate()
                .map(|(i, line)| {
                    if let Some(j) = line.find('S') {
                        start = Position(i, j);
                    }
                    line.chars().collect_vec()
                })
                .collect_vec(),
            start,
        ))
    }
}

impl Map {
    pub fn get_path_len(&mut self) -> IntType {
        let mut steps = 1;
        let mut curr = self.1.clone();
        let (init_c, init_prev_d) = self.get_connection();
        *self.pos_mut(&curr) = init_c;
        self.replace_curr_char(&curr);
        let mut prev_dir = curr.next(init_c, init_prev_d);
        let mut curr_char = self.pos(&curr);

        while curr_char != '#' && curr_char != '$' {
            steps += 1;
            self.replace_curr_char(&curr);
            prev_dir = curr.next(curr_char, prev_dir);
            curr_char = self.pos(&curr);
        }
        steps
    }

    pub fn pos(&self, p: &Position) -> char {
        self.0[p.0][p.1]
    }

    pub fn pos_mut(&mut self, p: &Position) -> &mut char {
        &mut self.0[p.0][p.1]
    }

    pub fn get_connection(&self) -> (char, Direction) {
        let (mut up, mut down, mut left, mut right) = (false, false, false, false);

        if self.1 .0 > 0 {
            let mut pos = self.1.clone();
            pos.next('|', Direction::Up);
            if self.pos(&pos) == '|' || self.pos(&pos) == '7' || self.pos(&pos) == 'F' {
                up = true;
            }
        }
        if self.1 .0 < self.0.len() - 1 {
            let mut pos = self.1.clone();
            pos.next('|', Direction::Down);
            if self.pos(&pos) == '|' || self.pos(&pos) == 'L' || self.pos(&pos) == 'J' {
                down = true;
            }
        }
        if self.1 .1 < self.0[0].len() - 1 {
            let mut pos = self.1.clone();
            pos.next('|', Direction::Right);
            if self.pos(&pos) == '-' || self.pos(&pos) == '7' || self.pos(&pos) == 'J' {
                right = true;
            }
        }
        if self.1 .1 > 0 {
            let mut pos = self.1.clone();
            pos.next('|', Direction::Left);
            if self.pos(&pos) == '-' || self.pos(&pos) == 'F' || self.pos(&pos) == 'L' {
                left = true;
            }
        }
        match (up, down, left, right) {
            (true, true, false, false) => ('|', Direction::Up),
            (true, false, true, false) => ('J', Direction::Right),
            (true, false, false, true) => ('L', Direction::Left),
            (false, true, true, false) => ('7', Direction::Right),
            (false, true, false, true) => ('F', Direction::Left),
            (false, false, true, true) => ('-', Direction::Left),
            _ => panic!(),
        }
    }

    pub fn replace_curr_char(&mut self, p: &Position) {
        let c = self.pos(p);
        *self.pos_mut(p) = match c {
            '|' => '#',
            'J' => '$',
            'L' => '$',
            'F' => '#',
            '7' => '#',
            _ => '$',
        };
    }

    pub fn count_enclosed(&self) -> IntType {
        self.0
            .iter()
            .map(|line| {
                line.iter()
                    .fold(
                        (0 as IntType, 0),
                        |(count, loop_layers_to_left), el| match el {
                            '$' => (count, loop_layers_to_left),
                            '#' => (count, loop_layers_to_left + 1),
                            _ => (
                                count
                                    + if loop_layers_to_left > 0 {
                                        loop_layers_to_left % 2
                                    } else {
                                        0
                                    },
                                loop_layers_to_left,
                            ),
                        },
                    )
                    .0
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
        assert_eq!(run(read("src/y23/d10/sample-a.txt").unwrap()).unwrap(), 8);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y23/d10/input.txt").unwrap()).unwrap(), 6882);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y23/d10/sample-b.txt").unwrap()).unwrap(), 8);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y23/d10/input.txt").unwrap()).unwrap(), 491);
    }
}
