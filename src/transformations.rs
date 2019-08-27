use crate::matrix::Matrix;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
  let t = Matrix::identity();
  let tt = t.update_at(0, 3, x).update_at(1, 3, y).update_at(2, 3, z);
  tt
}
pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
  let t = Matrix::identity();
  let tt = t.update_at(0, 0, x).update_at(1, 1, y).update_at(2, 2, z);
  tt
}
pub fn rotation_x(r: f64) -> Matrix {
  let t = Matrix::identity();
  let tt = t
    .update_at(1, 1, r.cos())
    .update_at(1, 2, -r.sin())
    .update_at(2, 1, r.sin())
    .update_at(2, 2, r.cos());
  tt
}
pub fn rotation_y(r: f64) -> Matrix {
  let t = Matrix::identity();
  let tt = t
    .update_at(0, 0, r.cos())
    .update_at(0, 2, r.sin())
    .update_at(2, 0, -r.sin())
    .update_at(2, 2, r.cos());
  tt
}
pub fn rotation_z(r: f64) -> Matrix {
  let t = Matrix::identity();
  let tt = t
    .update_at(0, 0, r.cos())
    .update_at(0, 1, -r.sin())
    .update_at(1, 0, r.sin())
    .update_at(1, 1, r.cos());
  tt
}

#[cfg(test)]

mod tests {
  use super::*;
  use crate::tuple::Tuple;

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
  #[test]
  fn a_scaling_matrix_applied_to_a_point() {
    let transform = scaling(2.0, 3.0, 4.0);
    let p = Tuple::point(-4.0, 6.0, 8.0);
    let expected = Tuple::point(-8.0, 18.0, 32.0);
    assert_eq!(expected, transform * p);
  }
  #[test]
  fn a_scaling_matrix_applied_to_a_vector() {
    let transform = scaling(2.0, 3.0, 4.0);
    let v = Tuple::vector(-4.0, 6.0, 8.0);
    let expected = Tuple::vector(-8.0, 18.0, 32.0);
    assert_eq!(expected, transform * v);
  }
  #[test]
  fn multiplying_by_the_inverse_of_a_scaling() {
    let transform = scaling(2.0, 3.0, 4.0);
    let inv = transform.inverse();
    let v = Tuple::vector(-4.0, 6.0, 8.0);
    let expected = Tuple::vector(-2.0, 2.0, 2.0);
    assert_eq!(expected, inv * v);
  }
  #[test]
  fn reflection_is_scaling_by_a_negative_value() {
    let transform = scaling(-1.0, 1.0, 1.0);
    let p = Tuple::point(2.0, 3.0, 4.0);
    let expected = Tuple::point(-2.0, 3.0, 4.0);
    assert_eq!(expected, transform * p);
  }

  #[test]
  fn rotating_a_point_around_the_x_axis() {
    let p = Tuple::point(0.0, 1.0, 0.0);
    let half_quarter = rotation_x(std::f64::consts::PI / 4.0);
    let full_quarter = rotation_x(std::f64::consts::PI / 2.0);
    let expected_half_quarter = Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0);
    let expected_full_quarter = Tuple::point(0.0, 0.0, 1.0);
    assert_eq!(expected_half_quarter, half_quarter * p);
    assert_eq!(expected_full_quarter, full_quarter * p);
  }
  #[test]
  fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
    let p = Tuple::point(0.0, 1.0, 0.0);
    let half_quarter_inv = rotation_x(std::f64::consts::PI / 4.0).inverse();
    let expected_half_quarter = Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
    assert_eq!(expected_half_quarter, half_quarter_inv * p);
  }
  #[test]
  fn rotating_a_point_around_the_y_axis() {
    let p = Tuple::point(0.0, 0.0, 1.0);
    let half_quarter = rotation_y(std::f64::consts::PI / 4.0);
    let full_quarter = rotation_y(std::f64::consts::PI / 2.0);
    let expected_half_quarter = Tuple::point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0);
    let expected_full_quarter = Tuple::point(1.0, 0.0, 0.0);
    assert_eq!(expected_half_quarter, half_quarter * p);
    assert_eq!(expected_full_quarter, full_quarter * p);
  }
  #[test]
  fn rotating_a_point_around_the_z_axis() {
    let p = Tuple::point(0.0, 1.0, 0.0);
    let half_quarter = rotation_z(std::f64::consts::PI / 4.0);
    let full_quarter = rotation_z(std::f64::consts::PI / 2.0);
    let expected_half_quarter = Tuple::point(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);
    let expected_full_quarter = Tuple::point(-1.0, 0.0, 0.0);
    assert_eq!(expected_half_quarter, half_quarter * p);
    assert_eq!(expected_full_quarter, full_quarter * p);
  }
}
