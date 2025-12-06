pub mod debug;
pub mod grid;
pub mod parse;
pub mod bucket;

pub trait ResultExt<T> {
    fn into_inner(self) -> T;
}

impl<T> ResultExt<T> for Result<T, T> {
    fn into_inner(self) -> T {
        let (Ok(x) | Err(x)) = self;
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
