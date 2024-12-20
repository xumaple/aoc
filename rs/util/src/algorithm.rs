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

pub fn astar_safe<N>(start: N) -> Option<(impl Iterator<Item = N>, N::Cost)>
where
    N: AStarNode + Eq + Hash + Clone,
{
    pathfinding::directed::astar::astar(
        &start,
        |node| node.next().collect_vec(),
        <N as AStarNode>::heuristic,
        <N as PathfindingNode>::is_goal,
    )
    .map(|(items, cost)| (items.into_iter(), cost))
}

pub fn astar_bag<N>(start: N) -> Option<(impl Iterator<Item = impl Iterator<Item = N>>, N::Cost)>
where
    N: AStarNode + Eq + Hash + Clone,
{
    pathfinding::directed::astar::astar_bag(
        &start,
        |node| node.next().collect_vec(),
        <N as AStarNode>::heuristic,
        <N as PathfindingNode>::is_goal,
    )
    .map(|(solutions, cost)| (solutions.map(|v| v.into_iter()), cost))
}

/// Searches through node list to find all starting nodes, then runs `astar_bag` on each
pub fn astar_bag_all_starts<N>(
    nodes: impl Iterator<Item = N>,
) -> impl Iterator<Item = (N, impl Iterator<Item = impl Iterator<Item = N>>, N::Cost)>
where
    N: AStarNode + Eq + Hash + Clone,
{
    nodes.filter_map(|n| {
        n.is_start()
            .then(|| astar_bag(n.clone()).map(|(solutions, cost)| (n, solutions, cost)))
            .flatten()
    })
}
