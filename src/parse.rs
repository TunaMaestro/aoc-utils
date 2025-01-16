use atoi::FromRadix10SignedChecked;

fn nums<I: FromRadix10SignedChecked>(input: &str, numerical: impl Fn(u8) -> bool) -> Vec<I> {
    input
        .as_bytes()
        .chunk_by(|&a, &b| numerical(a) == numerical(b))
        .filter(|x| x.get(0).map(|&x| numerical(x)).unwrap_or(false))
        // .inspect(|x| print!("<{}>", String::from_utf8((*x).to_owned()).unwrap()))
        .filter_map(|x| atoi::atoi::<I>(x))
        .collect()
}

/// Watch for proper numbers being skipped due to being too large for given integer
pub fn nums_positive<I: FromRadix10SignedChecked>(input: &str) -> Vec<I> {
    nums(input, |x| x.is_ascii_digit())
}

pub fn nums_signed<I: FromRadix10SignedChecked>(input: &str) -> Vec<I> {
    nums(input, |x| x.is_ascii_digit() || x == b'-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strange_nums() {
        let s = "\
Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";
        assert_eq!(nums_positive::<usize>(&s), [117440, 0, 0, 0, 3, 5, 4, 3, 0])
    }
}
