use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use lina::{point2, vec2, Point2, Vec2};

#[derive(Debug)]
pub struct Grid<C>(pub Vec<Vec<C>>);

pub type Point = Point2<i32>;

impl<C> Index<Point> for Grid<C> {
    type Output = C;

    /// Panics if the point is out of bounds
    fn index(&self, index: Point) -> &Self::Output {
        &self.0[index.y as usize][index.x as usize]
    }
}

impl<C> IndexMut<Point> for Grid<C> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.0[index.y as usize][index.x as usize]
    }
}

impl<C> Grid<C> {
    pub fn new(v: Vec<Vec<C>>) -> Grid<C> {
        Grid(v)
    }

    pub fn read(input: &str, cell: fn(char) -> C) -> Grid<C> {
        let g: Vec<Vec<C>> = input
            .trim()
            .split('\n')
            .map(|ln| ln.chars().into_iter().map(cell).collect())
            .collect();

        #[cfg(debug_assertions)]
        {
            let line_lens: Vec<usize> = g.iter().map(|x| x.len()).collect();
            if line_lens.len() > 0 {
                assert!(
                    line_lens.iter().all(|&x| x == line_lens[0]),
                    "All lines must be the same length"
                );
            }
        }
        Grid(g)
    }

    pub fn position(&self, test: fn(&C) -> bool) -> Option<Point> {
        let coords = self
            .0
            .iter()
            .enumerate()
            .map(|(i, &ref v)| (i, v.iter().position(|x: &C| test(x))))
            .find_map(|(y, x)| x.and_then(|x| Some(point2(x as i32, y as i32))));
        coords
    }

    pub fn contains(&self, coord: Point) -> bool {
        coord.x >= 0
            && coord.y >= 0
            && self.0.len() > 0
            && self.0.len() > coord.y as usize
            && self.0[0].len() > coord.x as usize
    }
    pub fn new_with_dimensions(dimension: Vec2<i32>, new: impl Fn(Point) -> C) -> Grid<C> {
        Grid::new(
            (0..dimension.y)
                .map(|y| (0..dimension.x).map(|x| new(point2(x, y))).collect())
                .collect(),
        )
    }

    pub fn dimension(&self) -> Vec2<i32> {
        if self.0.len() == 0 {
            vec2(0, 0)
        } else {
            vec2(self.0[0].len() as i32, self.0.len() as i32)
        }
    }

    pub fn map<T>(&self, f: impl Fn(&C) -> T) -> Grid<T> {
        return Grid(
            self.0
                .iter()
                .map(|x| x.iter().map(|x| f(x)).collect())
                .collect(),
        );
    }

    pub fn neighbours(&self, src: Point) -> Vec<(Point, &C)> {
        UP_RIGHT_DOWN_LEFT
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
        self.0
            .iter()
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
