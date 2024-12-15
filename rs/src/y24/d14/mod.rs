use abstract_grid::{Grid, GridBoundary};
use util::*;

pub mod a;
pub mod b;

pub type IntType = usize;

#[derive(Debug, Clone)]
struct Robot {
    max_x: usize,
    max_y: usize,
    pos: Position,
    v: SignedPosition,
}

impl Add<SignedPosition> for Robot {
    type Output = Self;
    fn add(mut self, rhs: SignedPosition) -> Self::Output {
        let new_pos = rhs + self.pos;
        // println!("{} {}", self.len() as isize, self.width() as isize);
        // println!("new_pos: {new_pos:?}; %= ({}, {})", new_pos.x.rem_euclid(self.len() as isize), new_pos.y.rem_euclid(self.width() as isize));
        self.pos.x = (new_pos.x.rem_euclid(self.max_x as isize)) as usize;
        self.pos.y = (new_pos.y.rem_euclid(self.max_y as isize)) as usize;
        self
    }
}

impl Robot {
    pub fn new(max_x: usize, max_y: usize, pos: (usize, usize), velocity: (isize, isize)) -> Self {
        Self {
            max_x,
            max_y,
            pos: pos.into(),
            v: velocity.into(),
        }
    }

    pub fn mv(self, time: isize) -> Self {
        let v_time = self.v * time;
        self + v_time
    }
}

#[derive(Debug)]
struct Library {
    robots: Vec<Robot>,
    len: usize,
    width: usize,
}

impl FromStr for Library {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (size, s) = s.ssplit_once('\n');
        let (len, width) = size.ssplit_once(',');
        let len = usize::ufrom(len);
        let width = usize::ufrom(width);
        let re = Regex::new(r"-?\d+").unwrap();
        Ok(Self {
            robots: s
                .lines()
                .map(|line| {
                    let mut it = re.find_iter(line);
                    Robot::new(
                        len,
                        width,
                        (it.unext().as_str().uinto(), it.unext().as_str().uinto()),
                        (it.unext().as_str().uinto(), it.unext().as_str().uinto()),
                    )
                })
                .collect_vec(),
            len,
            width,
        })
    }
}

impl Library {
    pub fn simulate(mut self, time: isize) -> Self {
        self.robots = self.robots.into_iter().map(|r| r.mv(time)).collect_vec();
        self
    }

    pub fn safety_factor(&mut self) -> IntType {
        let mid_x = self.len / 2;
        let mid_y = self.width / 2;

        let quadrant = |pos: Position| match (pos.x.cmp(&(mid_x)), pos.y.cmp(&(mid_y))) {
            (Ordering::Less, Ordering::Less) => Some(0),
            (Ordering::Less, Ordering::Greater) => Some(1),
            (Ordering::Greater, Ordering::Less) => Some(2),
            (Ordering::Greater, Ordering::Greater) => Some(3),
            _ => None,
        };

        self.robots
            .iter_mut()
            .filter_map(|robot| quadrant(robot.pos))
            .fold([0; 4], |mut counts, quadrant| {
                counts[quadrant] += 1;
                counts
            })
            .into_iter()
            .product()
    }

    pub fn has_christmas_tree(&mut self) -> bool {
        self.display().contains("########")
    }

    pub fn display(&mut self) -> String {
        let hm: HashSet<Position> = self.robots.iter().map(|r| r.pos).collect();
        (0..self.len)
            .into_iter()
            .map(move |x| {
                (0..self.width)
                    .into_iter()
                    .map(|y| {
                        if hm.contains(&Position::new(x, y)) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            })
            .join("\n")
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    // SAMPLE NEEDS TO CHANGE `grid_size` FUNCTION
    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d14/sample.txt").unwrap()).unwrap(), 12);
    }

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y24/d14/input.txt").unwrap()).unwrap(),
            220971520
        );
    }
}

#[cfg(test)]
mod test_b {
    use super::Library;
    use util::read;

    #[test]
    fn official() {
        assert_eq!(
            read("src/y24/d14/input.txt")
                .unwrap()
                .trim()
                .parse::<Library>()
                .unwrap()
                .simulate(6355)
                .display(),
            read("src/y24/d14/easter_egg.txt").unwrap().trim()
        );
    }
}
