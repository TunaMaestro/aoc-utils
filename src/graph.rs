// use std::{
//     collections::{HashMap, VecDeque},
//     hash::Hash,
//     ops::{Add, Sub},
// };

// use num::{Zero, zero};

// use crate::grid::{Grid, Point};

// pub trait Graph {
//     type Node: Hash + PartialEq + Eq + Clone + Copy;
//     type Weight: PartialEq + PartialOrd + Sub + Add + Zero;

//     fn adjacent(&self, v: Self::Node) -> Vec<(Self::Node, Self::Weight)>;

//     fn are_adjacent(&self, u: Self::Node, v: Self::Node) -> Option<Self::Weight>;

//     fn bfs(
//         &self,
//         src: Self::Node,
//     ) -> HashMap<Self::Node, HashMap<Self::Node, Option<Self::Weight>>> {
//         let mut queue: VecDeque<Self::Node> = VecDeque::new();
//         let mut visited: HashMap<Self::Node, HashMap<Self::Node, Option<Self::Weight>>> =
//             HashMap::new();
//         visited
//             .entry(src)
//             .or_insert(HashMap::new())
//             .insert(src, Some(zero()));
//         queue.push_back(src);
//         while let Some(u) = queue.pop_front() {
//             for v in self.adjacent(u) {
                
//             }
//         }
//         visited
//     }
// }

// pub struct GridGraph<C> {
//     grid: Grid<C>,
// }

// impl<C> GridGraph<C> {
//     pub fn new(grid: Grid<C>) -> GridGraph<C> {
//         GridGraph { grid }
//     }
// }

// impl<C> Graph for GridGraph<C>
// where
//     C: Copy + PartialOrd + PartialEq + Sub<Output = C> + num::Zero,
// {
//     type Node = Point;
//     type Weight = C;

//     fn adjacent(&self, v: Self::Node) -> Vec<(Self::Node, Self::Weight)> {
//         self.grid
//             .neighbours(v)
//             .iter()
//             .map(|&(a, b)| (a, Self::weight(self.grid[v], *b)))
//             .collect()
//     }

//     fn are_adjacent(&self, u: Self::Node, v: Self::Node) -> Option<Self::Weight> {
//         if u.distance2_from(v) > 1 {
//             None
//         } else {
//             Some(GridGraph::weight(self.grid[u], self.grid[v]))
//         }
//     }
// }

// impl<C> GridGraph<C>
// where
//     C: Sub<Output = C>,
// {
//     fn weight(u: C, v: C) -> C {
//         v - u
//     }
// }
