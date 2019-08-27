use crate::matrix::Matrix;
use crate::tuple::Tuple;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
  let mut t = Matrix::identity();
  let tt = t.update_at(0, 3, x).update_at(1, 3, y).update_at(2, 3, z);
  tt
}

#[cfg(test)]

mod tests {
  use super::*;

  #[test]
  fn multiplying_by_a_translation_matrix() {
    let transform = translation(5.0, -3.0, 2.0);
    let p = Tuple::point(-3.0, 4.0, 5.0);
    let expected = Tuple::point(2.0, 1.0, 7.0);
    assert_eq!(expected, transform * p);
  }
  #[test]
  fn multiplying_by_the_inverse_of_a_translation() {
    let transform = translation(5.0, -3.0, 2.0);
    let inv = transform.inverse();
    let p = Tuple::point(-3.0, 4.0, 5.0);
    let expected = Tuple::point(-8.0, 7.0, 3.0);
    assert_eq!(expected, inv * p);
  }
  #[test]
  fn translation_does_not_affect_vectors() {
    let transform = translation(5.0, -3.0, 2.0);
    let v = Tuple::vector(-3.0, 4.0, 5.0);
    let expected = v;
    assert_eq!(expected, transform * v);
  }
}
