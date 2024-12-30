use grid_vec::{Cursor, Grid};
use util::*;

pub mod a;
pub mod b;

pub type IntType = usize;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum NumSpace {
    N(u8),
    A,
    Noop,
}

impl UnsafeFrom<char> for NumSpace {
    fn ufrom(input: char) -> Self {
        if input.is_ascii_digit() {
            Self::N(input as u8)
        } else if input == 'A' {
            Self::A
        } else if input == ' ' {
            Self::Noop
        } else {
            panic!()
        }
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum ArrowSpace {
    Arrow(Direction),
    A,
    Noop,
}

impl UnsafeFrom<char> for ArrowSpace {
    fn ufrom(input: char) -> Self {
        match input {
            '<' => Self::Arrow(Direction::L),
            '^' => Self::Arrow(Direction::U),
            '>' => Self::Arrow(Direction::R),
            'v' => Self::Arrow(Direction::D),
            'A' => Self::A,
            ' ' => Self::Noop,
            _ => panic!(),
        }
    }
}

impl ArrowSpace {
    fn to_char(&self) -> char {
        match *self {
            Self::Arrow(Direction::L) => '<',
            Self::Arrow(Direction::U) => '^',
            Self::Arrow(Direction::R) => '>',
            Self::Arrow(Direction::D) => 'v',
            Self::A => 'A',
            Self::Noop => ' ',
        }
    }
}

impl Debug for ArrowSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

trait KeyPadSpace: Hash + Copy {
    fn can_be_traversed(self) -> bool;
}

impl KeyPadSpace for NumSpace {
    fn can_be_traversed(self) -> bool {
        self != Self::Noop
    }
}

impl KeyPadSpace for ArrowSpace {
    fn can_be_traversed(self) -> bool {
        self != Self::Noop
    }
}

struct KeyPad<T: KeyPadSpace> {
    grid: Grid<T>,
    positions: HashMap<T, SignedPosition>,
    ptr: SignedPosition,
    dp: HashMap<String, IntType>,
    inner: Option<Box<KeyPad<ArrowSpace>>>,
}

impl<T: KeyPadSpace + Clone + Eq> KeyPad<T> {
    fn new_from_grid(grid: Grid<T>, ptr: SignedPosition, layer: u8, depth: u8) -> Self {
        let positions = grid
            .iter_flat()
            .map(|cs| {
                let (pos, t) = cs.to_enumerated_tuple();
                (t, pos.into())
            })
            .collect();
        Self {
            ptr,
            positions,
            grid,
            dp: HashMap::new(),
            inner: (layer < depth - 1)
                .then(|| Box::new(KeyPad::<ArrowSpace>::new(layer + 1, depth))),
        }
    }

    fn get_str(opt: Option<(char, isize)>) -> String {
        opt.map(|(c, n)| vec![c; n as usize].into_iter().join(""))
            .unwrap_or_default()
    }

    fn can_be_traversed(s: &String, cs: Cursor<T>) -> bool {
        let mut cs = cs.clone();
        let b = s.chars().all(|arr| match ArrowSpace::ufrom(arr) {
            ArrowSpace::Arrow(dir) => cs.step(dir).unwrap().val().can_be_traversed(),
            _ => true,
        });
        b
    }

    fn type_arrow_pad(
        &mut self,
        shift: SignedPosition,
        start_pos: SignedPosition,
    ) -> Option<IntType> {
        if let Some(ref mut inner) = self.inner {
            let cs = self.grid.cursor(start_pos.into());
            let x = Self::get_str(match shift.x.cmp(&0) {
                Ordering::Equal => None,
                Ordering::Greater => Some(('v', shift.x)),
                Ordering::Less => Some(('^', -1 * shift.x)),
            });
            let y = Self::get_str(match shift.y.cmp(&0) {
                Ordering::Equal => None,
                Ordering::Greater => Some(('>', shift.y)),
                Ordering::Less => Some(('<', -1 * shift.y)),
            });
            let a = format!("{x}{y}A");
            let b = format!("{y}{x}A");

            let a_answer = Self::can_be_traversed(&a, cs)
                .then(|| inner.as_mut().type_str(a))
                .flatten();
            let b_answer = Self::can_be_traversed(&b, cs)
                .then(|| inner.as_mut().type_str(b))
                .flatten();

            match (a_answer, b_answer) {
                (Some(a), Some(b)) => Some(std::cmp::min(a, b)),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            }
        } else {
            Some((shift.x.abs() + shift.y.abs()) as IntType + 1)
        }
    }
}

impl KeyPad<NumSpace> {
    pub fn new(depth: u8) -> Self {
        Self::new_from_grid(
            "789\n456\n123\n 0A".parse().unwrap(),
            SignedPosition::new(3, 2),
            0,
            depth,
        )
    }

    pub fn type_char(&mut self, char: NumSpace) -> Option<IntType> {
        let next_ptr = *self.positions.get(&char).unwrap();
        let diff = next_ptr - self.ptr;
        self.ptr = next_ptr;
        self.type_arrow_pad(diff, next_ptr - diff)
    }
}

impl KeyPad<ArrowSpace> {
    pub fn new(layer: u8, depth: u8) -> Self {
        Self::new_from_grid(
            " ^A\n<v>".parse().unwrap(),
            SignedPosition::new(0, 2),
            layer,
            depth,
        )
    }

    pub fn type_str(&mut self, s: String) -> Option<IntType> {
        if let Some(answer) = self.dp.get(&s) {
            return Some(*answer);
        }

        let answer = s.chars().try_fold(0, |acc, c| {
            self.type_char(ArrowSpace::ufrom(c)).map(|n| acc + n)
        });
        if let Some(answer) = answer {
            self.dp.insert(s.clone(), answer);
        }
        answer
    }

    pub fn type_char(&mut self, char: ArrowSpace) -> Option<IntType> {
        let next_ptr = *self.positions.get(&char).unwrap();
        let diff = next_ptr - self.ptr;
        self.ptr = next_ptr;
        self.type_arrow_pad(diff, next_ptr - diff)
    }
}

fn robot_inputs(s: &str, depth: u8) -> IntType {
    let numeric: IntType = s[..3].uinto();
    let mut num_pad = KeyPad::<NumSpace>::new(depth);
    numeric
        * s.chars()
            .map(|c| num_pad.type_char(NumSpace::ufrom(c)).unwrap())
            .sum::<IntType>()
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(
            run(read("src/y24/d21/sample.txt").unwrap()).unwrap(),
            126384
        );
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y24/d21/input.txt").unwrap()).unwrap(), 134120);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(
            run(read("src/y24/d21/sample.txt").unwrap()).unwrap(),
            154115708116294
        );
    }

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y24/d21/input.txt").unwrap()).unwrap(),
            167389793580400
        );
    }
}
