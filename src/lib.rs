use crate::grid::Point;

pub mod bucket;
pub mod debug;
pub mod graph;
pub mod grid;
pub mod parse;
pub mod union_find;

pub trait ResultExt<T> {
    fn into_inner(self) -> T;
}

impl<T> ResultExt<T> for Result<T, T> {
    fn into_inner(self) -> T {
        let (Ok(x) | Err(x)) = self;
        x
    }
}

pub fn sort2<T: PartialOrd>((a, b): (T, T)) -> (T, T) {
    if a <= b { (a, b) } else { (b, a) }
}

trait MinMax {
    fn min(&self, other: &Self) -> Self;
    fn max(&self, other: &Self) -> Self;
}
impl MinMax for Point {
    fn min(&self, other: &Self) -> Self {
        Point::new(self.x.min(other.x), self.y.min(other.y))
    }

    fn max(&self, other: &Self) -> Self {
        Point::new(self.x.max(other.x), self.y.max(other.y))
    }
}

trait MinMaxIterator {
    type T;
    fn min_elementwise(self) -> Self::T;
    fn max_elementwise(self) -> Self::T;
}

impl<T, I> MinMaxIterator for I
where
    T: MinMax,
    I: Iterator<Item = T>,
{
    type T = T;
    fn min_elementwise(self) -> T {
        self.reduce(|a, b| a.min(&b))
            .expect("there is no MinMax of an iterator with zero length")
    }

    fn max_elementwise(self) -> T {
        self.reduce(|a, b| a.max(&b))
            .expect("there is no MinMax of an iterator with zero length")
    }
}

#[cfg(test)]
mod tests {}
