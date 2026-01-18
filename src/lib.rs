
use lina::{Point2, Scalar};
use num::{Num, Signed};

pub mod bucket;
pub mod debug;
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

pub trait MoreNormDistance {
    type Field;
    fn distance_manhattan(&self, other: Self) -> Self::Field;
    fn distance_inf(&self, other: Self) -> Self::Field;
}

impl<C: Signed + Num + Scalar + Ord> MoreNormDistance for Point2<C> {
    type Field = C;
    fn distance_manhattan(&self, other: Self) -> C {
        (*self - other)
            .map(|x| x.abs())
            .iter()
            .fold(C::zero(), |a, b| a + b)
    }

    fn distance_inf(&self, other: Self) -> C {
        (*self - other)
            .map(|x| x.abs())
            .iter()
            .max()
            .unwrap_or(C::zero())
    }
}

#[cfg(test)]
mod tests {}
