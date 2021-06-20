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
        // We panic if partial_cmp fails.
        other
            .minimum_total_cost()
            .partial_cmp(&self.minimum_total_cost())
            .unwrap()
    }
}

#[allow(dead_code)]
fn solve<N: Node>(start: N, goal: N) -> Option<(Vec<N>, N::Cost)> {
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

    solution.map(|p| {
        let mut current = &p.last_node;
        let mut path: Vec<MaybeUninit<N>> = (0..p.length).map(|_| MaybeUninit::uninit()).collect();
        for node in path.iter_mut().rev() {
            *node = MaybeUninit::new(current.node);
            match &current.prev {
                Some(c) => current = c,
                None => {}
            };
        }
        (
            unsafe {
                // SAFETY: All the elements in `path` have been initialized.
                mem::transmute(path)
            },
            p.cost,
        )
    })
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
    fn it_works() {
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
        assert!(
            (f64::from(cost) - ((6usize.pow(2) + 6usize.pow(2)) as f64).sqrt()).abs() < 0.00001
        );
    }
}
