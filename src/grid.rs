pub mod sparse;

use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use arrayvec::ArrayVec;
use lina::{Matrix, Point2, Vec2, point2, vec2};

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

pub trait GridTrait:
    Index<Point, Output = Self::Cell> + IndexMut<Point, Output = Self::Cell>
{
    type Cell;

    fn position(&self, test: fn(&Self::Cell) -> bool) -> Option<Point>;
    fn contains(&self, coord: Point) -> bool;
    fn dimension(&self) -> Vec2<i32>;

    fn adjacent(&self, src: Point) -> ArrayVec<(Point, &Self::Cell), 4>;

    fn iter_coordinates(&self) -> impl Iterator<Item = Point>;
    fn get(&self, p: Point) -> Option<&Self::Cell>;
    fn display(&self) -> String
    where
        Self::Cell: Display;
}

impl<C> GridTrait for Grid<C> {
    type Cell = C;

    fn position(&self, test: fn(&C) -> bool) -> Option<Point> {
        Grid::position(self, test)
    }

    fn contains(&self, coord: Point) -> bool {
        Grid::contains(self, coord)
    }

    fn dimension(&self) -> Vec2<i32> {
        Grid::dimension(self)
    }

    fn adjacent(&self, src: Point) -> ArrayVec<(Point, &C), 4> {
        Grid::adjacent(self, src)
    }

    fn iter_coordinates(&self) -> impl Iterator<Item = Point> {
        Grid::iter_coordinates(self)
    }

    fn get(&self, p: Point) -> Option<&C> {
        Grid::get(self, p)
    }

    fn display(&self) -> String
    where
        C: Display,
    {
        Grid::display(self)
    }
}

impl<C> Index<Point> for Grid<C> {
    type Output = C;

    /// Panics if the point is out of bounds
    fn index(&self, index: Point) -> &Self::Output {
        debug_assert!(self.contains(index));
        &self.inner[self.idx(index.y as usize, index.x as usize)]
    }
}

impl<C> IndexMut<Point> for Grid<C> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        debug_assert!(self.contains(index));
        let idx = self.idx(index.y as usize, index.x as usize);
        &mut self.inner[idx]
    }
}

impl<C> Index<Point2<usize>> for Grid<C> {
    type Output = C;

    /// Panics if the point is out of bounds
    fn index(&self, index: Point2<usize>) -> &Self::Output {
        debug_assert!(self.contains(index.map(|x| x as i32)));
        &self.inner[self.idx(index.y, index.x)]
    }
}

impl<C> IndexMut<Point2<usize>> for Grid<C> {
    fn index_mut(&mut self, index: Point2<usize>) -> &mut Self::Output {
        debug_assert!(self.contains(index.map(|x| x as i32)));
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

impl<C: PartialEq> PartialEq for Grid<C> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner && self.width == other.width
    }
}

pub struct TransformGrid<'a, C> {
    matrix: Matrix<i32, 2, 2>,
    matrix_inv: Matrix<i32, 2, 2>,
    grid: &'a mut Grid<C>,
}

fn transform_keep_positive_quadrant(
    dimension: Vec2<i32>,
    matrix: Matrix<i32, 2, 2>,
    point: Point,
) -> Point {
    let dimension = dimension - Vec2::new(1, 1);
    let transformed_idx = matrix.transform(point.to_vec());
    let dim_tfn = matrix.transform(dimension);
    let abs_dim_tfn = dim_tfn.map(|x| x.abs());
    let offset = ((abs_dim_tfn - dim_tfn) / 2).to_point();
    let positive_positive_idx = offset + transformed_idx;
    positive_positive_idx
}

impl<'a, C> TransformGrid<'a, C> {
    fn transform_point(&self, index: Point) -> Point {
        transform_keep_positive_quadrant(self.grid.dimension(), self.matrix, index)
    }

    fn inverse_point(&self, index: Point) -> Point {
        transform_keep_positive_quadrant(self.dimension(), self.matrix_inv, index)
    }

    pub fn from_grid(grid: &'a mut Grid<C>, transform: Matrix<i32, 2, 2>) -> Self {
        let (a, b, c, d) = (
            transform.elem(0, 0),
            transform.elem(0, 1),
            transform.elem(1, 0),
            transform.elem(1, 1),
        );
        let det = a * d - b * c;

        assert!(det == 1 || det == -1, "Transform matrix must be unitary");

        let inv = Matrix::from_rows([[d, -b], [-c, a]]) * det;

        TransformGrid {
            matrix: transform,
            matrix_inv: inv,
            grid,
        }
    }
}

impl<'a> TransformGrid<'a, ()> {
    pub fn rot(count: usize) -> Matrix<i32, 2, 2> {
        let id = Matrix::identity();
        let rot = Matrix::from_rows([[0, -1], [1, 0]]);
        (0..count).fold(id, |a, _| rot * a)
    }
}

impl<'a, C> Index<Point> for TransformGrid<'a, C> {
    type Output = C;

    /// Panics if the point is out of bounds
    fn index(&self, index: Point) -> &Self::Output {
        self.grid.index(self.inverse_point(index))
    }
}

impl<'a, C> IndexMut<Point> for TransformGrid<'a, C> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        self.grid.index_mut(self.inverse_point(index))
    }
}

impl<'a, C> Index<Point2<usize>> for TransformGrid<'a, C> {
    type Output = C;

    /// Panics if the point is out of bounds
    fn index(&self, index: Point2<usize>) -> &Self::Output {
        self.grid.index(
            self.inverse_point(index.map(|x| x as i32))
                .map(|x| x as usize),
        )
    }
}

impl<'a, C> IndexMut<Point2<usize>> for TransformGrid<'a, C> {
    fn index_mut(&mut self, index: Point2<usize>) -> &mut Self::Output {
        self.grid.index_mut(
            self.inverse_point(index.map(|x| x as i32))
                .map(|x| x as usize),
        )
    }
}

impl<'a, C> GridTrait for TransformGrid<'a, C> {
    type Cell = C;

    fn position(&self, test: fn(&Self::Cell) -> bool) -> Option<Point> {
        self.grid.position(test).map(|p| self.transform_point(p))
    }

    fn contains(&self, coord: Point) -> bool {
        self.grid.contains(self.inverse_point(coord))
    }

    fn dimension(&self) -> Vec2<i32> {
        self.matrix
            .transform(self.grid.dimension())
            .map(|x| x.abs())
    }

    fn adjacent(&self, src: Point) -> ArrayVec<(Point, &Self::Cell), 4> {
        self.grid.adjacent(self.inverse_point(src))
    }

    fn iter_coordinates(&self) -> impl Iterator<Item = Point> {
        PointIterator::new(self.dimension())
    }

    fn get(&self, p: Point) -> Option<&Self::Cell> {
        self.grid.get(self.inverse_point(p))
    }

    fn display(&self) -> String
    where
        Self::Cell: Display,
    {
        let mut s = String::new();
        dbg!(self.dimension(), self.grid.dimension());
        for y in 0..self.dimension().y {
            for x in 0..self.dimension().x {
                let p = Point::new(x, y);
                dbg!(p, self.inverse_point(p));
                s += &format!("{}", self[p]);
            }
            s += "\n";
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use lina::{Matrix, point2, vec2};

    use crate::grid::{Grid, GridTrait, TransformGrid};

    use super::*;

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

    #[test]
    fn test_rots() {
        assert_eq!(TransformGrid::<()>::rot(0), Matrix::identity());
        assert_eq!(
            TransformGrid::<()>::rot(1),
            Matrix::from_rows([[0, -1], [1, 0]])
        );
        assert_eq!(
            TransformGrid::<()>::rot(3),
            Matrix::from_rows([[0, 1], [-1, 0]])
        );
    }

    #[test]
    fn test_tranform_grid() {
        let mut g = Grid::read(
            &"\
...........
...#...#...
....#.#....
.....#.....
....O......
...O.......
...........
",
            |x| x,
        );

        let t_g = TransformGrid::from_grid(&mut g, TransformGrid::rot(1));

        let p = t_g.display();
        assert_eq!(
            &p,
            "\
.......
.......
.......
.O...#.
..O.#..
...#...
....#..
.....#.
.......
.......
.......
"
        );
    }

    #[test]
    fn test_grid_point_transform() {
        let mut g = Grid::new_with_dimensions_uniform(Vec2::new(5, 9), ());

        let tfn = TransformGrid::from_grid(&mut g, TransformGrid::rot(1));

        assert_eq!(tfn.dimension(), Vec2::new(9, 5));
        assert_eq!(tfn.transform_point(Point::new(0, 0)), Point::new(8, 0));
        assert_eq!(tfn.transform_point(Point::new(4, 8)), Point::new(0, 4));
        assert_eq!(tfn.transform_point(Point::new(2, 4)), Point::new(4, 2));

        assert_eq!(tfn.inverse_point(Point::new(8, 0)), Point::new(0, 0));
        assert_eq!(tfn.inverse_point(Point::new(0, 4)), Point::new(4, 8));
        assert_eq!(tfn.inverse_point(Point::new(4, 2)), Point::new(2, 4));
    }
}
