use util::*;

pub mod a;
pub mod b;

pub type IntType = usize;

trait GraphCursor: Hash + Clone + Eq + Debug {}

#[derive(Clone, Copy)]
struct Cursor<T: GraphCursor> {
    pub index: usize,
    graph: *const UndirectedGraph<T>,
}

impl<T: GraphCursor> Debug for Cursor<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}: {:?})",
            self.index,
            self.clone().connected_indices().collect_vec()
        )
    }
}

impl<T: GraphCursor> Cursor<T> {
    pub fn new(index: usize, graph: *const UndirectedGraph<T>) -> Self {
        Self { index, graph }
    }

    pub fn val(&self) -> &T {
        unsafe { &(*self.graph).values.get(self.index).unwrap() }
    }

    fn connected_indices(self) -> impl Iterator<Item = usize> {
        unsafe { (*self.graph).edges.get_vals(self.index).copied() }
    }

    pub fn next(self) -> impl Iterator<Item = Self>
    where
        T: 'static,
    {
        let ptr = self.graph.clone();
        self.connected_indices()
            .map(move |index| unsafe { (*ptr).cursor(index) })
    }
}

#[derive(Clone, Copy)]
struct CursorMut<T: GraphCursor> {
    pub index: usize,
    graph: *mut UndirectedGraph<T>,
}

impl<T: GraphCursor> Debug for CursorMut<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}: {:?})",
            self.index,
            self.connected_indices().collect_vec()
        )
    }
}

impl<T: GraphCursor> CursorMut<T> {
    pub fn new(index: usize, graph: *mut UndirectedGraph<T>) -> Self {
        Self { index, graph }
    }

    pub fn val(&self) -> &T {
        unsafe { &(*self.graph).values.get(self.index).unwrap() }
    }

    pub fn val_mut(&mut self) -> &mut T {
        unsafe { &mut (*self.graph).values[self.index] }
    }

    fn connected_indices(&self) -> impl Iterator<Item = usize> {
        unsafe { (*self.graph).edges.get_vals(self.index).copied() }
    }

    pub fn next(&self) -> impl Iterator<Item = Self> + use<'_, T> {
        self.connected_indices()
            .map(|index| unsafe { (*self.graph).cursor_mut(index) })
    }
}

#[derive(Debug)]
struct UndirectedGraph<T: GraphCursor> {
    values: Vec<T>,
    edges: MultiMap<usize, usize>,
    node_indices: HashMap<T, usize>,
}

impl<T: GraphCursor> UndirectedGraph<T> {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            edges: MultiMap::new(),
            node_indices: HashMap::new(),
        }
    }

    pub fn cursor(&self, index: usize) -> Cursor<T> {
        Cursor::new(index, self)
    }

    pub fn cursor_mut(&mut self, index: usize) -> CursorMut<T> {
        CursorMut::new(index, self)
    }

    pub fn get_or_new_index(&mut self, value: T) -> usize {
        if let Some(index) = self.node_indices.get(&value).copied() {
            index
        } else {
            let index = self.values.len();
            self.node_indices.insert(value.clone(), index);
            self.values.push(value);
            index
        }
    }

    pub fn add_edge(&mut self, a: T, b: T) {
        let a_idx = self.get_or_new_index(a);
        let b_idx = self.get_or_new_index(b);
        self.edges.insert(a_idx, b_idx);
        self.edges.insert(b_idx, a_idx);
    }
}

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
struct Computer {
    name: String,
    visited: Option<usize>,
}

impl Computer {
    pub fn new(name: String) -> Self {
        Self {
            name,
            visited: None,
        }
    }
}

impl GraphCursor for Computer {}

struct NetworkMap(UndirectedGraph<Computer>);

impl FromStr for NetworkMap {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = UndirectedGraph::new();
        s.lines().for_each(|line| {
            let (a, b) = line.ssplit_once('-');
            graph.add_edge(Computer::new(a.to_string()), Computer::new(b.to_string()));
        });
        Ok(Self(graph))
    }
}

impl NetworkMap {
    fn trip_cliques(start: Cursor<Computer>) -> impl Iterator<Item = Vec<usize>> {
        start.clone().next().flat_map(move |adj1| {
            adj1.clone()
                .next()
                .filter_map(move |adj2| {
                    adj2.clone()
                        .connected_indices()
                        .contains(&start.index)
                        .then(|| {
                            let mut v = vec![start.index, adj1.index, adj2.index];
                            v.sort();
                            v
                        })
                })
                .collect_vec()
                .into_iter()
        })
    }
    pub fn find_historian_trip_cliques(&self) -> IntType {
        (0..self.0.values.len())
            .filter_map(|index| {
                let cs = self.0.cursor(index);
                cs.val()
                    .name
                    .starts_with('t')
                    .then(|| Self::trip_cliques(cs))
            })
            .flatten()
            .collect::<HashSet<_>>()
            .len()
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d23/sample.txt").unwrap()).unwrap(), 7);
    }

    // #[test]
    // fn offical() {
    //     assert_eq!(run(read("src/y24/d23/input.txt").unwrap()).unwrap(), 0);
    // }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d23/sample.txt").unwrap()).unwrap(), 0);
    }

    // #[test]
    // fn offical() {
    //     assert_eq!(run(read("src/y24/d23/input.txt").unwrap()).unwrap(), 0);
    // }
}
