use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use arrayvec::ArrayVec;
use lina::{Point2, Vec2, point2, vec2};

#[derive(Debug)]
pub struct Grid<C> {
    inner: Vec<C>,
    width: usize,
}

impl<C> Grid<C> {
    fn idx(&self, y: usize, x: usize) -> usize {
        return y * self.width + x;
    }
}

pub type Point = Point2<i32>;

impl<C> Index<Point> for Grid<C> {
    type Output = C;

    /// Panics if the point is out of bounds
    fn index(&self, index: Point) -> &Self::Output {
        &self.inner[self.idx(index.y as usize, index.x as usize)]
    }
}

impl<C> IndexMut<Point> for Grid<C> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        let idx = self.idx(index.y as usize, index.x as usize);
        &mut self.inner[idx]
    }
}

impl<C> Index<Point2<usize>> for Grid<C> {
    type Output = C;

    /// Panics if the point is out of bounds
    fn index(&self, index: Point2<usize>) -> &Self::Output {
        &self.inner[self.idx(index.y, index.x)]
    }
}

impl<C> IndexMut<Point2<usize>> for Grid<C> {
    fn index_mut(&mut self, index: Point2<usize>) -> &mut Self::Output {
        let idx = self.idx(index.y, index.x);
        &mut self.inner[idx]
    }
}

impl<C> Grid<C> {
    pub fn new(v: Vec<Vec<C>>) -> Grid<C> {
        let width = v[0].len();
        Grid {
            inner: v.into_iter().flatten().collect(),
            width,
        }
    }

    pub fn read(input: &str, cell: fn(char) -> C) -> Grid<C> {
        let width = input
            .find("\n")
            .expect("Expect a newline character to signify width");
        let g: Vec<C> = input
            .trim()
            .split('\n')
            .map(|ln| ln.chars().into_iter().map(cell))
            .flatten()
            .collect();

        #[cfg(debug_assertions)]
        {
            let line_lens: Vec<usize> = input.trim().split("\n").map(|x| x.len()).collect();
            if line_lens.len() > 0 {
                let lens_str = line_lens
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                assert!(
                    line_lens.iter().all(|&x| x == line_lens[0] && x == width),
                    "All lines must be the same length: {}",
                    lens_str
                );
            }
        }
        Grid { inner: g, width }
    }

    pub fn position(&self, test: fn(&C) -> bool) -> Option<Point> {
        self.iter_coordinates().filter(|x| test(&self[*x])).next()
    }

    pub fn contains(&self, coord: Point) -> bool {
        coord.x >= 0 && coord.y >= 0 && self.dimension().y > coord.y && self.dimension().x > coord.x
    }
    pub fn new_with_dimensions(dimension: Vec2<i32>, new: impl Fn(Point) -> C) -> Grid<C> {
        Grid::new(
            (0..dimension.y)
                .map(|y| (0..dimension.x).map(|x| new(point2(x, y))).collect())
                .collect(),
        )
    }

    pub fn dimension(&self) -> Vec2<i32> {
        if self.inner.len() == 0 {
            vec2(0, 0)
        } else {
            vec2(self.width as i32, (self.inner.len() / self.width) as i32)
        }
    }

    pub fn map<T>(&self, f: impl Fn(&C) -> T) -> Grid<T> {
        return Grid {
            inner: self.inner.iter().map(|x| f(x)).collect(),
            width: self.width,
        };
    }

    pub fn adjacent(&self, src: Point) -> ArrayVec<(Point, &C), 4> {
        UP_RIGHT_DOWN_LEFT
            .iter()
            .map(|&d| src + d)
            .filter(|&n| self.contains(n))
            .map(|p| (p, &self[p]))
            .collect()
    }

    pub fn neighbours(&self, src: Point) -> ArrayVec<(Point, &C), 8> {
        NEIGHBOURS
            .iter()
            .map(|&d| src + d)
            .filter(|&n| self.contains(n))
            .map(|p| (p, &self[p]))
            .collect()
    }

    pub fn iter_coordinates(&self) -> PointIterator {
        PointIterator::new(self.dimension())
    }

    pub fn get(&self, p: Point) -> Option<&C> {
        if self.contains(p) {
            Some(&self[p])
        } else {
            None
        }
    }
}

pub struct PointIterator {
    dim: Vec2<i32>,
    p: Point,
}

impl PointIterator {
    fn new(dim: Vec2<i32>) -> Self {
        Self {
            dim,
            p: point2(0, 0),
        }
    }
}

impl Iterator for PointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let p = self.p;
        self.p.x += 1;
        if self.p.x == self.dim.x {
            self.p.x = 0;
            self.p.y += 1;
        }

        if p.x < self.dim.x && p.y < self.dim.y {
            Some(p)
        } else {
            None
        }
    }
    //
}

impl<C> Grid<C>
where
    C: std::convert::Into<char> + Copy,
{
    pub fn char(&self) -> Grid<char> {
        self.map(|&x| x.into())
    }
}

impl<C> Grid<C>
where
    C: Default + Copy,
{
    pub fn get_or_default(&self, p: Point) -> C {
        self.get(p).copied().unwrap_or_default()
    }
}
impl<T> Grid<T>
where
    T: Display,
{
    pub fn display(&self) -> String {
        self.inner
            .chunks(self.width)
            .map(|x| x.iter().map(|x| format!("{x}")).collect())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn print(&self) {
        println!("{}", self.display());
    }
}

impl<C: Copy> Grid<C> {
    pub fn new_with_dimensions_uniform(dimension: Vec2<i32>, new: C) -> Grid<C> {
        Grid::new_with_dimensions(dimension, |_| new)
    }
}

pub const UP_RIGHT_DOWN_LEFT: [Vec2<i32>; 4] = [vec2(0, -1), vec2(1, 0), vec2(0, 1), vec2(-1, 0)];
pub const NEIGHBOURS: [Vec2<i32>; 8] = [
    vec2(-1, -1),
    vec2(-1, 0),
    vec2(-1, 1),
    vec2(0, -1),
    vec2(0, 1),
    vec2(1, -1),
    vec2(1, 0),
    vec2(1, 1),
];

pub fn orthogonal_to_index(dir: Vec2<i32>) -> Option<usize> {
    let (x, y) = (dir.x, dir.y);
    if !(x == 0 || y == 0) {
        return None;
    }
    // x is ±1 XOR y is ±1
    if !((x.abs() == 1) != (y.abs() == 1)) {
        return None;
    }

    let i = y - x + 1 + x.abs();
    // trust
    /*
     * x | y | y - x + 1 | |x| | Out
     * 0 | -1|         0 |  0  |   0
     * 1 | 0 |         0 |  1  |   1
     * 0 | 1 |         2 |  0  |   2
     * -1| 0 |         2 |  1  |   3
     */
    assert!(0 <= i && i < 4);
    Some(i as usize)
}

#[cfg(test)]
mod tests {
    use lina::{point2, vec2};

    use crate::grid::Grid;

    use super::Point;

    #[test]
    fn test_grid() {
        let s = "###\n...\n...";
        let g = Grid::read(s, |x| x);

        let p: Point = point2(1, 1);
        let v = vec2(1, -1);

        assert_eq!(g[p], '.');
        assert_eq!(g[p + v], '#');
    }

    #[test]
    fn test_outside() {
        let s = "###\n...\n...";
        let g = Grid::read(s, |x| x);

        let p: Point = point2(1, 1);
        let v = vec2(1, -1);

        assert!(!g.contains(p + 8 * v))
    }
}
