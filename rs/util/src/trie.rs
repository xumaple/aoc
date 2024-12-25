use crate::*;

pub trait StartNode: Sized + PartialEq + Eq + PartialOrd + Ord + Copy + Debug {
    fn start() -> Self;
    fn is_start(&self) -> bool;
    fn display(&self) -> String;
}

impl StartNode for u8 {
    fn start() -> Self {
        u8::MAX
    }
    fn is_start(&self) -> bool {
        *self == u8::MAX
    }
    fn display(&self) -> String {
        match *self {
            u8::MAX => "START".to_owned(),
            _ => format!("{}", *self as char),
        }
    }
}

#[derive(Debug)]
pub struct Trie<T: StartNode> {
    start: RcCell<Node<T>>,
}

pub struct Node<T: StartNode> {
    next: BTreeMap<T, RcCell<Node<T>>>,
    val: T,
}

impl<T: StartNode> Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:?}: {})",
            self.val.display(),
            self.next
                .iter()
                .map(|(val, node)| match val.is_start() {
                    true => val.display(),
                    false => format!("{:?}", node.deref().borrow()),
                })
                .join(", ")
        )
    }
}

impl<T: StartNode> Trie<T> {
    pub fn new() -> Self {
        Self {
            start: Rc::new(RefCell::new(Node::start())),
        }
    }

    pub fn insert<I: Iterator<Item = T>>(&mut self, iter: I) {
        self.start
            .deref()
            .borrow_mut()
            .insert(iter, self.start.clone());
    }

    pub fn contains<I: Iterator<Item = T> + Clone>(&self, iter: I) -> bool {
        self.start.deref().borrow().contains(iter)
    }
}

impl<I, T> FromIterator<I> for Trie<T>
where
    T: StartNode,
    I: Iterator<Item = T>,
{
    fn from_iter<It: IntoIterator<Item = I>>(iter: It) -> Self {
        let mut trie = Trie::new();
        let mut iter = iter.into_iter();
        while let Some(next) = iter.next() {
            trie.insert(next);
        }
        trie
    }
}

impl<T: StartNode> Node<T> {
    pub fn start() -> Self {
        Self::new(T::start())
    }

    pub fn new(val: T) -> Self {
        Self {
            next: BTreeMap::new(),
            val,
        }
    }

    pub fn insert<I: Iterator<Item = T>>(&mut self, mut iter: I, start: RcCell<Self>) {
        if let Some(val) = iter.next() {
            let next = self
                .next
                .entry(val)
                .or_insert_with(|| Rc::new(RefCell::new(Node::new(val))));
            (**next).borrow_mut().insert(iter, start);
        } else {
            self.next.entry(T::start()).or_insert(start);
        }
    }

    pub fn contains<I: Iterator<Item = T> + Clone>(&self, mut iter: I) -> bool {
        if let Some(val) = iter.next() {
            if let Some(node) = self.next.get(&val) {
                if (**node).borrow().contains(iter.clone()) {
                    return true;
                }
            }
            if let Some(node) = self.next.get(&T::start()) {
                if let Some(node) = (**node).borrow().next.get(&val) {
                    if (**node).borrow().contains(iter) {
                        return true;
                    }
                }
            }
            false
        } else {
            true
        }
    }
}
