use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct Tuple(f64, f64, f64, f64);

impl Tuple {
    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple(x, y, z, 1.0)
    }
    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple(x, y, z, 0.0)
    }
}

impl Add for Tuple {
    type Output = Tuple;
    fn add(self, other: Tuple) -> Tuple {
        Tuple (
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
        )
    }
}

impl Sub for Tuple {
    type Output = Tuple;
    fn sub(self, other: Tuple) -> Tuple {
        Tuple(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
        )
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        let eps = 1e-6;
        (self.0 - other.0).abs() < eps
        && (self.1 - other.1).abs() < eps
        && (self.2 - other.2).abs() < eps
        && (self.3 - other.3).abs() < eps
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
    #[test]
    fn add_two_tuples() {
        let t1 = Tuple(3.0, -2.0, 5.0, 1.0);
        let t2 = Tuple(-2.0, 3.0, 1.0, 0.0);
        let expected = Tuple(1.0, 1.0, 6.0, 1.0);
        let actual = t1 + t2;
        assert_eq!(expected, actual);
    }
    #[test]
    fn subtract_two_points() {
        let t1 = Tuple::point(3.0, 2.0, 1.0);
        let t2 = Tuple::point(5.0, 6.0, 7.0);
        let expected = Tuple::vector(-2.0, -4.0, -6.0);
        let actual = t1 - t2;
        assert_eq!(expected, actual);
    }
}
