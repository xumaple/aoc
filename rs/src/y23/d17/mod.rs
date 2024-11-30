use util::*;

pub mod a;
pub mod b;

pub type IntType = u32;

pub struct HeatMap {
    grid: Grid<IntType>,
    min_moves_forward: usize,
    max_moves_forward: usize,
}

impl HeatMap {
    pub fn new(s: &str, min_moves_forward: usize, max_moves_forward: usize) -> Self {
        Self {
            grid: s.parse().unwrap(),
            min_moves_forward,
            max_moves_forward,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HeatNode {
    pos: Position<IntType>,
    dir: Direction,
    moves: usize,
}

impl HeatNode {
    fn new(pos: Position<IntType>, dir: Direction, moves: usize) -> Self {
        Self { pos, dir, moves }
    }
}

impl Dijkstras<HeatNode, IntType> for HeatMap {
    fn neighbors(&self, node: &HeatNode) -> Vec<(IntType, HeatNode)> {
        [node.dir, node.dir.turn_L(), node.dir.turn_R()][if node.moves < self.min_moves_forward {
            0..1
        } else {
            if node.moves >= self.max_moves_forward {
                if node.moves == 4 {
                    panic!();
                }
                1..3
            } else {
                0..3
            }
        }]
        .iter()
        .filter_map(|&dir| {
            if let Some(new_pos) = node.pos.next(dir) {
                Some((
                    self.grid[new_pos],
                    HeatNode::new(
                        new_pos,
                        dir,
                        if dir == node.dir { node.moves + 1 } else { 1 },
                    ),
                ))
            } else {
                None
            }
        })
        .collect_vec()
    }

    fn is_goal(&self, node: &HeatNode) -> bool {
        node.pos.x == self.grid.len() - 1 && node.pos.y == self.grid.width() - 1
    }
}

impl HeatMap {
    pub fn run_algo(&self) -> Result<IntType, E> {
        let mut last_node = self.run(HeatNode::new(
            Position::new(0, 0, Some(&self.grid)),
            Direction::R,
            0,
        ))?;
        let ans = last_node.cost.clone();

        let mut grid: Grid<IntType> = Grid::new(vec![vec![0; self.grid.width()]; self.grid.len()]);

        while let Some(prev) = last_node.clone().prev {
            grid[last_node.inner().pos] = last_node.cost;
            let prev = (*prev.borrow()).clone();
            last_node = prev;
        }
        debug(grid);

        Ok(ans)
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y23/d17/sample.txt").unwrap()).unwrap(), 0);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y23/d17/input.txt").unwrap()).unwrap(), 916);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y23/d17/sample.txt").unwrap()).unwrap(), 0);
    }

    // #[test]
    // fn offical() {
    //     assert_eq!(run(read("src/y23/d17/input.txt").unwrap()).unwrap(), 0);
    // }
}
