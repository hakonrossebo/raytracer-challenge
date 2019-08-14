pub struct Tuple(f64, f64, f64, f64);

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn tuple_w1_is_a_point() {
        let tp = Tuple(4.3, -4.2, 3.1, 1.0);
        let Tuple(_, _, _, w) = tp;
        assert_eq!(1.0, w);
    }
}
