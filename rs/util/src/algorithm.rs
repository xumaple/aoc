use super::*;
use pathfinding;

pub trait CostFunction
where
    Self: Number
        + Copy
        + Clone
        + num_traits::Zero
        + std::ops::AddAssign
        + Sized
        + Ord
        + PartialOrd
        + PartialEq
        + Eq
        + Add<Output = Self>,
{
    fn inf() -> Self;
}

pub trait PathfindingNode: Sized {
    type Cost: CostFunction;
    fn next(&self) -> impl Iterator<Item = (Self, Self::Cost)>;
    fn is_start(&self) -> bool;
    fn is_goal(&self) -> bool;
}

pub trait AStarNode: PathfindingNode {
    fn heuristic(&self) -> Self::Cost;
}

pub fn astar<N>(start: N) -> (impl Iterator<Item = N>, N::Cost)
where
    N: AStarNode + Eq + Hash + Clone,
{
    let (items, cost) = pathfinding::directed::astar::astar(
        &start,
        |node| node.next().collect_vec(),
        <N as AStarNode>::heuristic,
        <N as PathfindingNode>::is_goal,
    )
    .unwrap();
    (items.into_iter(), cost)
}

pub fn astar_bag<N>(start: N) -> (impl Iterator<Item = impl Iterator<Item = N>>, N::Cost)
where
    N: AStarNode + Eq + Hash + Clone,
{
    let (solutions, cost) = pathfinding::directed::astar::astar_bag(
        &start,
        |node| node.next().collect_vec(),
        <N as AStarNode>::heuristic,
        <N as PathfindingNode>::is_goal,
    )
    .unwrap();
    (solutions.map(|v| v.into_iter()), cost)
}
