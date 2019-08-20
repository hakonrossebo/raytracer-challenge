use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Tuple(pub f64, pub f64, pub f64, pub f64);

impl Tuple {
    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple(x, y, z, 1.0)
    }
    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple(x, y, z, 0.0)
    }
    fn magnitude(self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2 + self.3 * self.3).sqrt()
    }
    pub fn normalize(self) -> Tuple {
        let m = self.magnitude();
        Tuple(self.0 / m, self.1 / m, self.2 / m, self.3 / m)
    }
    pub fn dot(self, other: Tuple) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2 + self.3 * other.3
    }
    pub fn cross(self, other: Tuple) -> Tuple {
        Tuple::vector(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }
}

impl Add for Tuple {
    type Output = Tuple;
    fn add(self, other: Tuple) -> Tuple {
        Tuple(
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

impl Neg for Tuple {
    type Output = Tuple;
    fn neg(self) -> Tuple {
        Tuple(-self.0, -self.1, -self.2, -self.3)
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;
    fn mul(self, f: f64) -> Tuple {
        Tuple(self.0 * f, self.1 * f, self.2 * f, self.3 * f)
    }
}
impl Div<f64> for Tuple {
    type Output = Tuple;
    fn div(self, f: f64) -> Tuple {
        Tuple(self.0 / f, self.1 / f, self.2 / f, self.3 / f)
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
    #[test]
    fn subtract_vector_from_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);
        let expected = Tuple::point(-2.0, -4.0, -6.0);
        let actual = p - v;
        assert_eq!(expected, actual);
    }
    #[test]
    fn subtract_two_vectors() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);
        let expected = Tuple::vector(-2.0, -4.0, -6.0);
        let actual = v1 - v2;
        assert_eq!(expected, actual);
    }
    #[test]
    fn subtract_vector_from_zero_vector() {
        let v_zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);
        let expected = Tuple::vector(-1.0, 2.0, -3.0);
        let actual = v_zero - v;
        assert_eq!(expected, actual);
    }
    #[test]
    fn negating_a_tuple() {
        let a = Tuple(1.0, -2.0, 3.0, -4.0);
        let expected = Tuple(-1.0, 2.0, -3.0, 4.0);
        assert_eq!(expected, -a);
    }
    #[test]
    fn multiplying_by_scalar() {
        let a = Tuple(1.0, -2.0, 3.0, -4.0);
        let expected = Tuple(3.5, -7.0, 10.5, -14.0);
        let actual = a * 3.5;
        assert_eq!(expected, actual);
    }
    #[test]
    fn multiplying_by_fraction() {
        let a = Tuple(1.0, -2.0, 3.0, -4.0);
        let expected = Tuple(0.5, -1.0, 1.5, -2.0);
        let actual = a * 0.5;
        assert_eq!(expected, actual);
    }
    #[test]
    fn dividing_tuple_by_scalar() {
        let a = Tuple(1.0, -2.0, 3.0, -4.0);
        let expected = Tuple(0.5, -1.0, 1.5, -2.0);
        let actual = a / 2.0;
        assert_eq!(expected, actual);
    }
    #[test]
    fn magnitude_of_vector() {
        assert_eq!(1.0, Tuple::vector(1.0, 0.0, 0.0).magnitude());
        assert_eq!(1.0, Tuple::vector(0.0, 1.0, 0.0).magnitude());
        assert_eq!(1.0, Tuple::vector(0.0, 0.0, 1.0).magnitude());
        assert_eq!(14.0_f64.sqrt(), Tuple::vector(1.0, 2.0, 3.0).magnitude());
        assert_eq!(14.0_f64.sqrt(), Tuple::vector(-1.0, -2.0, -3.0).magnitude());
    }
    #[test]
    fn normalize_vector() {
        assert_eq!(
            Tuple::vector(1.0, 0.0, 0.0).normalize(),
            Tuple::vector(1.0, 0.0, 0.0)
        );
        assert_eq!(
            Tuple::vector(0.0, 1.0, 0.0).normalize(),
            Tuple::vector(0.0, 1.0, 0.0)
        );
        assert_eq!(
            Tuple::vector(0.0, 0.0, 1.0).normalize(),
            Tuple::vector(0.0, 0.0, 1.0)
        );
        let v = Tuple::vector(1.0, 2.0, 3.0);
        let nv = v.magnitude();
        let v2 = Tuple::vector(1.0 / nv, 2.0 / nv, 3.0 / nv);
        assert_eq!(v.normalize(), v2);
    }
    #[test]
    fn magnitude_of_normalized_vector() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(v.normalize().magnitude(), 1.0);
    }
    #[test]
    fn dot_product_of_two_tuples() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(a.dot(b), 20.0);
    }
    #[test]
    fn cross_product_of_two_tuples() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(a.cross(b), Tuple::vector(-1.0, 2.0, -1.0));
    }
}
