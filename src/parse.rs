use atoi::FromRadix10SignedChecked;

fn nums<I: FromRadix10SignedChecked>(input: &str, numerical: impl Fn(u8) -> bool) -> Vec<I> {
    input
        .as_bytes()
        .chunk_by(|&a, &b| numerical(a) == numerical(b))
        .filter(|x| x.get(0).map(|&x| numerical(x)).unwrap_or(false))
        .filter_map(|x| atoi::atoi::<I>(x))
        .collect()
}

pub fn nums_positive<I: FromRadix10SignedChecked>(input: &str) -> Vec<I> {
    nums(input, |x| x.is_ascii_digit())
}

pub fn nums_signed<I: FromRadix10SignedChecked>(input: &str) -> Vec<I> {
    nums(input, |x| x.is_ascii_digit() || x == b'-')
}
