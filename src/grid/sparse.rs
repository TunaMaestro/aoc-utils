use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};


use lina::vec2;

use crate::{MinMaxIterator, 
    grid::{Grid, GridTrait, Point, UP_RIGHT_DOWN_LEFT}}
;

#[derive(Clone)]
pub struct SparseGrid<C> {
    inner: HashMap<Point, C>,
    default: C,
}

impl<C: Clone> SparseGrid<C> {
    pub fn new(default: C) -> Self {
        Self {
            inner: HashMap::new(),
            default,
        }
    }
}

impl<C: Clone> Index<Point> for SparseGrid<C> {
    type Output = C;

    /// Panics if the point is out of bounds
    fn index(&self, index: Point) -> &Self::Output {
        &self.inner.get(&index).unwrap_or(&self.default)
    }
}

impl<C: Clone> IndexMut<Point> for SparseGrid<C> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        self.inner
            .entry(index)
            .or_insert_with(|| self.default.clone())
    }
}

impl<C: Clone> GridTrait for SparseGrid<C> {
    type Cell = C;

    fn position(&self, test: fn(&Self::Cell) -> bool) -> Option<super::Point> {
        self.inner.iter().find(|(_, c)| test(c)).map(|res| *res.0)
    }

    fn contains(&self, coord: super::Point) -> bool {
        self.inner.contains_key(&coord)
    }

    fn dimension(&self) -> lina::Vec2<i32> {
        self.inner.keys().copied().max_elementwise().to_vec()
            - self.inner.keys().copied().min_elementwise().to_vec()
            + vec2(1, 1)
    }

    fn adjacent(&self, src: super::Point) -> arrayvec::ArrayVec<(super::Point, &Self::Cell), 4> {
        UP_RIGHT_DOWN_LEFT
            .into_iter()
            .map(|v| src + v)
            .map(|p| (p, &self[p]))
            .collect()
    }

    fn iter_coordinates(&self) -> impl Iterator<Item = Point> {
        self.inner.keys().copied()
    }

    fn get(&self, p: super::Point) -> Option<&Self::Cell> {
        Some(self.inner.get(&p).unwrap_or(&self.default))
    }

    fn display(&self) -> String
    where
        Self::Cell: std::fmt::Display,
    {
        todo!()
    }
}

impl<C: Copy> From<SparseGrid<C>> for Grid<C> {
    fn from(value: SparseGrid<C>) -> Self {
        let dim = value.dimension();

        let min = value.inner.keys().copied().min_elementwise();

        let mut grid = Grid::new_with_dimensions_uniform(dim, value.default);

        for (p, c) in value.inner {
            grid[(p - min).to_point()] = c;
        }

        grid
    }
}
