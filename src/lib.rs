use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    hash::Hash,
    iter,
    ops::Add,
};

pub trait Node: Copy + Eq + Hash {
    type AdjacentNodesIterator: Iterator<Item = Self>;
    type Cost: Add<Output = <Self as Node>::Cost> + Copy + Default + Eq + Ord;
    fn adjacent(&self) -> <Self as Node>::AdjacentNodesIterator;
    fn move_cost(&self, next: &Self) -> <Self as Node>::Cost;
    fn minimum_remaining_cost(&self, goal: &Self) -> <Self as Node>::Cost;
}

struct Path<N: Node> {
    nodes: Vec<N>,
    cost: N::Cost,
    goal: N,
}

impl<N: Node> Path<N> {
    fn new(start: N, goal: N) -> Self {
        Self {
            nodes: vec![start],
            cost: Default::default(),
            goal,
        }
    }
    fn last(&self) -> N {
        self.nodes[self.nodes.len() - 1]
    }
    fn minimum_total_cost(&self) -> N::Cost {
        self.cost + self.last().minimum_remaining_cost(&self.goal)
    }
    fn next_move(&self, node: N) -> Self {
        let mut nodes = Vec::with_capacity(self.nodes.len() + 1);
        nodes.extend(self.nodes.iter().map(|n| *n).chain(iter::once(node)));
        let cost = self.cost + self.last().move_cost(&node);
        Self {
            nodes,
            cost,
            goal: self.goal,
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
        other.minimum_total_cost().cmp(&self.minimum_total_cost())
    }
}

#[allow(dead_code)]
fn solve<N: Node>(start: N, goal: N) -> Option<(Vec<N>, N::Cost)> {
    let mut solution = None;
    let mut paths = BinaryHeap::new();
    let mut visited = HashSet::new();

    paths.push(Path::new(start, goal));

    while let Some(path) = paths.pop() {
        let current = path.nodes[path.nodes.len() - 1];
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

    solution.map(|p| (p.nodes, p.cost))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    struct MyFloat(f64);

    impl Add for MyFloat {
        type Output = Self;
        fn add(self, other: Self) -> Self {
            MyFloat(self.0 + other.0)
        }
    }

    impl Default for MyFloat {
        fn default() -> Self {
            MyFloat(0.0)
        }
    }

    impl Eq for MyFloat {}

    impl Ord for MyFloat {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

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
        type Cost = MyFloat;
        fn adjacent(&self) -> Self::AdjacentNodesIterator {
            AdjacentPositionsIter::new(*self)
        }
        fn move_cost(&self, next: &Self) -> Self::Cost {
            MyFloat((((self.0 - next.0).pow(2) + (self.1 - next.1).pow(2)) as f64).sqrt())
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
        assert!((cost.0 - ((6usize.pow(2) + 6usize.pow(2)) as f64).sqrt()).abs() < 0.00001);
    }
}
