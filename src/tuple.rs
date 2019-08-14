pub struct Tuple(f64, f64, f64, f64);

impl Tuple {
    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple(x, y, z, 1.0)
    }
    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple(x, y, z, 0.0)
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn tuple_w1_is_a_point() {
        let tp = Tuple(4.3, -4.2, 3.1, 1.0);
        let Tuple(_, _, _, w) = tp;
        assert_eq!(1.0, w);
    }
    #[test]
    fn tuple_w0_is_a_vector() {
        let x_in = 4.3;
        let y_in = -4.2;
        let z_in = 3.1;
        let tp = Tuple(x_in, y_in, z_in, 0.0);
        let Tuple(_, _, _, w) = tp;
        assert_eq!(0.0, w);
    }
    #[test]
    fn point_creates_a_tuple_with_w1() {
        let x_in = 4.3;
        let y_in = -4.2;
        let z_in = 3.1;
        let p = Tuple::point(x_in, y_in, z_in);
        let Tuple(x, y, z, w) = p;
        assert_eq!(x_in, x);
        assert_eq!(y_in, y);
        assert_eq!(z_in, z);
        assert_eq!(1.0, w);
    }
    #[test]
    fn vector_creates_a_tuple_with_w0() {
        let x_in = 4.3;
        let y_in = -4.2;
        let z_in = 3.1;
        let p = Tuple::vector(x_in, y_in, z_in);
        let Tuple(x, y, z, w) = p;
        assert_eq!(x_in, x);
        assert_eq!(y_in, y);
        assert_eq!(z_in, z);
        assert_eq!(0.0, w);
    }
}
