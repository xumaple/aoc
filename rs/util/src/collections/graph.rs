use crate::*;

pub trait GraphCursor: Hash + Clone + Eq + Debug {}

#[derive(Clone, Copy)]
pub struct Cursor<T: GraphCursor> {
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

    pub fn connected_indices(&self) -> impl Iterator<Item = usize> {
        unsafe { (*self.graph).edges.get_vals(self.index).copied() }
    }

    pub fn next(&self) -> impl Iterator<Item = Self> {
        let ptr = self.graph.clone();
        self.connected_indices()
            .map(move |index| unsafe { (*ptr).cursor(index) })
    }
}

#[derive(Clone, Copy)]
pub struct CursorMut<T: GraphCursor> {
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

    pub fn connected_indices(&self) -> impl Iterator<Item = usize> {
        unsafe { (*self.graph).edges.get_vals(self.index).copied() }
    }

    pub fn next(&self) -> impl Iterator<Item = Self> + use<'_, T> {
        self.connected_indices()
            .map(|index| unsafe { (*self.graph).cursor_mut(index) })
    }
}

#[derive(Debug)]
pub struct UndirectedGraph<T: GraphCursor> {
    pub values: Vec<T>,
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
