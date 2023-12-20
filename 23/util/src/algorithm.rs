use super::*;
use priority_queue::PriorityQueue as PQ;

pub trait CostFunction
where
    Self: Number
        + Clone
        + std::ops::AddAssign
        + Sized
        + Ord
        + PartialOrd
        + PartialEq
        + Eq
        + Add<Output = Self>,
{
    fn inf() -> Self;
    fn zero() -> Self;
}

#[derive(Clone)]
pub struct DijkstrasNode<T, C>
where
    T: Clone + Copy + Debug + Hash + PartialEq + Eq,
    C: CostFunction,
{
    pub prev: Option<RcCell<DijkstrasNode<T, C>>>,
    pub inner: RcCell<T>,
    pub cost: C,
}

impl<T, C> Debug for DijkstrasNode<T, C>
where
    T: Clone + Copy + Debug + Hash + PartialEq + Eq,
    C: CostFunction,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.prev {
            Some(cell) => write!(
                f,
                "DijkNode {{\n  prev: {:?}, inner: {:?}, cost: {:?}\n}}",
                &(*cell.borrow()).inner(),
                self.inner(),
                self.cost
            ),
            None => write!(
                f,
                "DijkNode {{ prev: None\n  inner: {:?}\n  cost: {:?} }}",
                self.inner(),
                self.cost
            ),
        }
    }
}

impl<T, C> From<T> for DijkstrasNode<T, C>
where
    T: Clone + Copy + Debug + Hash + PartialEq + Eq,
    C: CostFunction,
{
    fn from(inner: T) -> Self {
        Self {
            prev: None,
            inner: RcCell::new(inner),
            cost: C::inf(),
        }
    }
}

impl<T, C> PartialEq for DijkstrasNode<T, C>
where
    T: Clone + Copy + Debug + Hash + PartialEq + Eq,
    C: CostFunction,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}
impl<T, C> Eq for DijkstrasNode<T, C>
where
    T: Clone + Copy + Debug + Hash + PartialEq + Eq,
    C: CostFunction,
{
}
impl<T, C> PartialOrd for DijkstrasNode<T, C>
where
    T: Clone + Copy + Debug + Hash + PartialEq + Eq,
    C: CostFunction,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // self.cost.partial_cmp(&other.cost)
        match self.cost.partial_cmp(&other.cost) {
            Some(Ordering::Greater) => Some(Ordering::Less),
            Some(Ordering::Equal) => Some(Ordering::Equal),
            Some(Ordering::Less) => Some(Ordering::Greater),
            None => None,
        }
    }
}
impl<T, C> Ord for DijkstrasNode<T, C>
where
    T: Clone + Copy + Debug + Hash + PartialEq + Eq,
    C: CostFunction,
{
    fn cmp(&self, other: &Self) -> Ordering {
        // self.cost.cmp(&other.cost)
        match self.cost.cmp(&other.cost) {
            Ordering::Greater => Ordering::Less,
            Ordering::Equal => Ordering::Equal,
            Ordering::Less => Ordering::Greater,
        }
    }
}
impl<T, C> Hash for DijkstrasNode<T, C>
where
    T: Clone + Copy + Debug + Hash + PartialEq + Eq,
    C: CostFunction,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.borrow().hash(state);
    }
}

impl<T, C> DijkstrasNode<T, C>
where
    T: Clone + Copy + Debug + Hash + PartialEq + Eq,
    C: CostFunction,
{
    pub fn inner(&self) -> Ref<T> {
        self.inner.borrow()
    }

    pub fn inner_mut(&mut self) -> RefMut<T> {
        self.inner.borrow_mut()
    }
}

pub trait Dijkstras<T, C>
where
    T: Clone + Copy + Debug + Hash + PartialEq + Eq, // Item
    C: CostFunction + Copy,
{
    fn neighbors(&self, node: &T) -> Vec<(C, T)>;
    fn is_goal(&self, node: &T) -> bool;

    fn run(&self, start: T) -> Result<DijkstrasNode<T, C>, E> {
        // let nodes = nodes.into_iter().map(DijkstrasNode::from).collect_vec();
        let mut start = DijkstrasNode::from(start);
        start.cost = C::zero();
        let mut nodes_map: HashMap<T, DijkstrasNode<T, C>> = HashMap::new();
        let mut heap = PQ::new();
        heap.push(start.clone(), start);

        let mut i = 0;
        let n = loop {
            i += 1;
            // if i > 20 { break C::inf(); }
            if heap.len() == 0 {
                return Err(E::AlgorithmError(
                    "Did not find goal node, no more elements in heap",
                ));
            }
            // println!("{:?}", heap.clone().into_sorted_vec());
            let (n, _) = heap.pop().unwrap();
            // println!("{:?}", n);
            if self.is_goal(&n.inner()) {
                break n;
            }
            let _ = self
                .neighbors(&n.inner())
                .iter()
                .map(|&(cost, m)| {
                    let m = nodes_map.entry(m).or_insert_with(|| DijkstrasNode::from(m));
                    let new_cost = n.cost + cost;
                    if new_cost < m.cost {
                        // println!("curr: {:?}; next: {:?}; cost: {:?}; new_cost: {:?}", n.inner(), m.inner(), m.cost, new_cost);
                        let mut m = m.clone();
                        m.cost = new_cost;
                        m.prev = Some(RcCell::from(n.clone()));
                        let _ = heap.push_decrease(m.clone(), m);
                    }
                    Ok(())
                })
                .collect::<Result<Vec<()>, E>>()?;
        };

        Ok(n)
    }
}
