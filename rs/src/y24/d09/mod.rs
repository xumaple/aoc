use util::*;

pub mod a;
pub mod b;

pub type IntType = usize;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Block {
    Contiguous(usize, usize), // val, len
    Chain(Box<Block>, Box<Block>),
    Empty(usize), // len
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Contiguous(val, len) => write!(f, "{}", vec![val.to_string(); *len].join("")),
            Self::Chain(b1, b2) => write!(f, "{:?}{:?}", *b1, *b2),
            Self::Empty(len) => write!(f, "{}", vec!["."; *len].join("")),
        }
    }
}

impl Block {
    pub fn new(amount: usize) -> Self {
        Self::Empty(amount)
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Contiguous(_, len) => *len,
            Self::Empty(len) => *len,
            Self::Chain(b1, b2) => b1.len() + b2.len(),
        }
    }

    pub fn filled_len(&self) -> usize {
        match self {
            Self::Contiguous(_, len) => *len,
            Self::Empty(_) => 0,
            Self::Chain(b1, b2) => b1.filled_len() + b2.filled_len(),
        }
    }

    pub fn unfilled_len(&self) -> usize {
        self.len() - self.filled_len()
    }

    pub fn is_full(&self) -> bool {
        match self {
            Self::Contiguous(_, _) => true,
            Self::Chain(_, b2) => b2.is_full(),
            Self::Empty(len) => *len == 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Empty(_) => true,
            _ => false,
        }
    }

    pub fn push(self, new: Self) -> Self {
        if new.filled_len() == 0 {
            return self;
        }
        match self {
            Self::Contiguous(_, _) => panic!(),
            Self::Chain(b1, b2) => Self::Chain(b1, Box::new(b2.push(new))),
            Self::Empty(len) => {
                let new_len = new.len();
                match len.cmp(&new_len) {
                    Ordering::Equal => new,
                    Ordering::Less => panic!(),
                    Ordering::Greater => {
                        Self::Chain(Box::new(new), Box::new(Self::Empty(len - new_len)))
                    }
                }
            }
        }
    }

    // Returns `amount` of values, if possible. If `amount` is greater than `self.filled_len()`
    // then it returns everything it has.
    pub fn pop(&mut self, amount: usize) -> Self {
        if self.is_empty() {
            return self.clone();
        }
        if self.filled_len() <= amount {
            let mut empty = Self::Empty(self.len());
            std::mem::swap(self, &mut empty);
            return empty;
        }
        match self {
            Self::Contiguous(val, len) => {
                *len -= amount;
                Self::Contiguous(*val, amount)
            }
            Self::Chain(b1, b2) => {
                let mut ret = Self::new(amount).push(b2.pop(amount));
                if !ret.is_full() {
                    let amount_left = amount - ret.filled_len();
                    ret = ret.push(b1.pop(amount_left));
                }
                ret
            }
            _ => unreachable!(),
        }
    }

    pub fn checksum(&self, starting_index: usize) -> IntType {
        match self {
            Self::Contiguous(val, len) => {
                val * (starting_index..starting_index + len).sum::<IntType>()
            }
            Self::Chain(b1, b2) => {
                b1.checksum(starting_index) + b2.checksum(starting_index + b1.filled_len())
            }
            Self::Empty(_) => 0,
        }
    }
}

struct FileSystem(Vec<Block>);

impl FromStr for FileSystem {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(FileSystem(
            s.chars()
                .enumerate()
                .map(|(i, c)| match i % 2 {
                    0 => Block::Contiguous(i / 2, u32::ufrom(c).uinton()),
                    1 => Block::Empty(u32::ufrom(c).uinton()),
                    _ => unreachable!(),
                })
                .collect(),
        ))
    }
}

impl Index<usize> for FileSystem {
    type Output = Block;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for FileSystem {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl FileSystem {
    fn move_block(&mut self, from: usize, to: usize, amount: usize) {
        let mut temp = std::mem::replace(&mut self[to], Block::new(0));
        temp = temp.push(self[from].pop(amount));
        let _ = std::mem::replace(&mut self[to], temp);
    }

    fn move_entire_block(&mut self, from: usize, to: usize) {
        self.move_block(from, to, self[from].len())
    }

    pub fn fill_up(mut self) -> Self {
        let mut cursor = 0;
        let mut back_cursor = self.0.len() - 1;

        loop {
            if self[cursor].is_full() {
                cursor += 1;
            } else {
                let amount_left = self[cursor].unfilled_len();
                while self[back_cursor].is_empty() {
                    back_cursor -= 1;
                }

                self.move_block(back_cursor, cursor, amount_left);
            }
            if cursor >= back_cursor {
                return self;
            }
        }
    }

    pub fn fill_wholes(mut self) -> Self {
        let mut full_cursor = 0;
        let mut cursor;
        let mut back_cursor = self.0.len() - 1;

        'outer: loop {
            while full_cursor <= back_cursor && self[full_cursor].is_full() {
                full_cursor += 1;
            }
            while back_cursor > 0 && self[back_cursor].is_empty() {
                back_cursor -= 1;
            }
            if full_cursor >= back_cursor {
                return self
            }
            cursor = full_cursor;
            let back_len = self[back_cursor].len();

            while self[cursor].unfilled_len() < back_len {
                cursor += 1;
                if cursor >= back_cursor {
                    back_cursor -= 1;
                    continue 'outer;
                }
            }

            self.move_entire_block(back_cursor, cursor);
        }
    }

    pub fn checksum(&self) -> IntType {
        let mut idx = 0;
        self.0
            .iter()
            .map(|block| {
                let checksum = block.checksum(idx);
                idx += block.len();
                checksum
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
        assert_eq!(run(read("src/y24/d09/sample.txt").unwrap()).unwrap(), 1928);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y24/d09/input.txt").unwrap()).unwrap(), 6370402949053);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d09/sample.txt").unwrap()).unwrap(), 2858);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y24/d09/input.txt").unwrap()).unwrap(), 6398096697992);
    }
}
