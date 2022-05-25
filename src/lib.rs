use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    hash::Hash,
    mem::{self, MaybeUninit},
    ops::Add,
    rc::Rc,
};

pub trait Node: Copy + Eq + Hash {
    type AdjacentNodesIterator: Iterator<Item = Self>;
    type Cost: Add<Output = <Self as Node>::Cost> + Copy + Default + PartialEq + PartialOrd;
    fn adjacent(&self) -> <Self as Node>::AdjacentNodesIterator;
    fn move_cost(&self, next: &Self) -> <Self as Node>::Cost;
    fn minimum_remaining_cost(&self, goal: &Self) -> <Self as Node>::Cost;
    fn cmp_costs(lhs: &<Self as Node>::Cost, rhs: &<Self as Node>::Cost) -> Ordering {
        // By default we just panic if partial_cmp fails.
        lhs.partial_cmp(rhs)
            .expect("Node::Cost::partial_cmp returned None")
    }
}

struct PathNode<N: Node> {
    pub node: N,
    pub prev: Option<Rc<PathNode<N>>>,
}

struct Path<N: Node> {
    last_node: Rc<PathNode<N>>,
    cost: N::Cost,
    goal: N,
    length: usize,
}

struct PathIterator<'a, N: Node> {
    current_node: Option<&'a Rc<PathNode<N>>>,
}

impl<N: Node> Path<N> {
    fn new(start: N, goal: N) -> Self {
        Self {
            last_node: Rc::new(PathNode {
                node: start,
                prev: None,
            }),
            cost: Default::default(),
            goal,
            length: 1,
        }
    }
    fn last(&self) -> N {
        self.last_node.node
    }
    fn minimum_total_cost(&self) -> N::Cost {
        self.cost + self.last().minimum_remaining_cost(&self.goal)
    }
    fn next_move(&self, node: N) -> Self {
        let last_node = Rc::new(PathNode {
            node,
            prev: Some(Rc::clone(&self.last_node)),
        });
        let cost = self.cost + self.last().move_cost(&node);
        Self {
            last_node,
            cost,
            goal: self.goal,
            length: self.length + 1,
        }
    }
    fn iter(&self) -> impl Iterator<Item = N> + '_ {
        PathIterator {
            current_node: Some(&self.last_node),
        }
    }
    fn as_vec(&self) -> Vec<N> {
        unsafe {
            // SAFETY: `self.iter()` should return `self.length` elements, so all the
            // elements in `path` should be initialized when we call `mem::transmute(path)`.
            let mut path: Vec<MaybeUninit<N>> =
                (0..self.length).map(|_| MaybeUninit::uninit()).collect();
            path.iter_mut()
                .rev()
                .zip(self.iter())
                .for_each(|(ptr, node)| *ptr = MaybeUninit::new(node));
            mem::transmute(path)
        }
    }
}

impl<'a, N: Node> Iterator for PathIterator<'a, N> {
    type Item = N;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current_node.take() {
            Some(n) => {
                self.current_node = n.prev.as_ref();
                Some(n.node)
            }
            None => None,
        }
    }
}

impl<N: Node> PartialEq for Path<N> {
    fn eq(&self, other: &Self) -> bool {
        self.minimum_total_cost() == other.minimum_total_cost()
    }
}

impl<N: Node> Eq for Path<N> {}

impl<N: Node> PartialOrd for Path<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<N: Node> Ord for Path<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        N::cmp_costs(&other.minimum_total_cost(), &self.minimum_total_cost())
    }
}

#[allow(dead_code)]
pub fn solve<N: Node>(start: N, goal: N) -> Option<(Vec<N>, N::Cost)> {
    let mut solution = None;
    let mut paths = BinaryHeap::new();
    let mut visited = HashSet::new();

    paths.push(Path::new(start, goal));

    while let Some(path) = paths.pop() {
        let current = path.last();
        let _ = visited.insert(current);
        if current == goal {
            solution = Some(path);
            break;
        }
        for n in current.adjacent() {
            if !visited.contains(&n) {
                paths.push(path.next_move(n));
            }
        }
    }

    solution.map(|p| (p.as_vec(), p.cost))
}

#[allow(dead_code)]
pub fn solve_all<N: Node>(start: N, goal: N) -> Vec<(Vec<N>, N::Cost)> {
    let mut solutions = Vec::new();
    let mut cost = None;
    let mut paths = BinaryHeap::new();

    paths.push(Path::new(start, goal));

    while let Some(path) = paths.pop() {
        let current = path.last();
        if current == goal {
            match cost {
                Some(cost) => {
                    if cost < path.cost {
                        break;
                    }
                }
                None => cost = Some(path.cost),
            }
            solutions.push(path);
        } else {
            for n in current.adjacent() {
                paths.push(path.next_move(n));
            }
        }
    }

    solutions
        .into_iter()
        .map(|p| (p.as_vec(), p.cost))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Position(isize, isize);

    struct AdjacentPositionsIter(Position, u8);

    impl AdjacentPositionsIter {
        fn new(position: Position) -> Self {
            AdjacentPositionsIter(position, 0)
        }
    }

    impl Iterator for AdjacentPositionsIter {
        type Item = Position;
        fn next(&mut self) -> Option<Self::Item> {
            let i = self.1;
            self.1 += 1;
            match i {
                0 => Some(Position(self.0 .0 + 1, self.0 .1 + 1)),
                1 => Some(Position(self.0 .0 + 1, self.0 .1)),
                2 => Some(Position(self.0 .0 + 1, self.0 .1 - 1)),
                3 => Some(Position(self.0 .0, self.0 .1 + 1)),
                4 => Some(Position(self.0 .0, self.0 .1 - 1)),
                5 => Some(Position(self.0 .0 - 1, self.0 .1 + 1)),
                6 => Some(Position(self.0 .0 - 1, self.0 .1)),
                7 => Some(Position(self.0 .0 - 1, self.0 .1 - 1)),
                _ => None,
            }
        }
    }

    impl Node for Position {
        type AdjacentNodesIterator = AdjacentPositionsIter;
        type Cost = f64;
        fn adjacent(&self) -> Self::AdjacentNodesIterator {
            AdjacentPositionsIter::new(*self)
        }
        fn move_cost(&self, next: &Self) -> Self::Cost {
            (((self.0 - next.0).pow(2) + (self.1 - next.1).pow(2)) as f64).sqrt()
        }
        fn minimum_remaining_cost(&self, goal: &Self) -> Self::Cost {
            self.move_cost(goal)
        }
    }

    #[test]
    fn test_solve() {
        let (path, cost) = solve(Position(-4, 5), Position(2, -1)).unwrap();
        assert_eq!(
            path,
            vec![
                Position(-4, 5),
                Position(-3, 4),
                Position(-2, 3),
                Position(-1, 2),
                Position(0, 1),
                Position(1, 0),
                Position(2, -1),
            ]
        );
        assert!((cost - ((6usize.pow(2) + 6usize.pow(2)) as f64).sqrt()).abs() < 0.00001);
    }

    #[test]
    fn test_solve_all() {
        let solutions = solve_all(Position(1, 1), Position(2, 3));
        assert_eq!(solutions.len(), 2);
        solutions
            .iter()
            .map(|s| s.1)
            .for_each(|cost| assert!((cost - 2f64.sqrt() - 1f64).abs() < 0.00001));
    }
}
