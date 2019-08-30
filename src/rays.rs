use crate::matrix::Matrix;
use crate::tuple::Tuple;

pub struct Ray {
  pub origin: Tuple,
  pub direction: Tuple,
}

impl Ray {
  pub fn new(o: Tuple, d: Tuple) -> Ray {
    Ray {
      origin: o,
      direction: d,
    }
  }

  pub fn position(&self, t: f64) -> Tuple {
    self.origin + self.direction * t
  }

  pub fn transform(&self, m: Matrix) -> Ray {
    Ray {
      //TODO:Avoid clone?
      origin: m.clone() * self.origin,
      direction: m.clone() * self.direction,
    }
  }
}

#[cfg(test)]

mod tests {
  use super::*;
  use crate::transformations::{scaling, translation};
  use crate::tuple::Tuple;
  #[test]
  fn creating_and_querying_a_ray() {
    let origin = Tuple::point(1.0, 2.0, 3.0);
    let direction = Tuple::vector(4.0, 5.0, 6.0);
    let r = Ray::new(origin, direction);
    assert_eq!(origin, r.origin);
    assert_eq!(direction, r.direction);
  }
  #[test]
  fn computing_a_point_from_a_distance() {
    let origin = Tuple::point(2.0, 3.0, 4.0);
    let direction = Tuple::vector(1.0, 0.0, 0.0);
    let r = Ray::new(origin, direction);
    let e0 = Tuple::point(2.0, 3.0, 4.0);
    let e1 = Tuple::point(3.0, 3.0, 4.0);
    let e2 = Tuple::point(1.0, 3.0, 4.0);
    let e3 = Tuple::point(4.5, 3.0, 4.0);
    let p0 = r.position(0.0);
    let p1 = r.position(1.0);
    let p2 = r.position(-1.0);
    let p3 = r.position(2.5);
    assert_eq!(e0, p0);
    assert_eq!(e1, p1);
    assert_eq!(e2, p2);
    assert_eq!(e3, p3);
  }
  #[test]
  fn translating_a_ray() {
    let origin = Tuple::point(1.0, 2.0, 3.0);
    let direction = Tuple::vector(0.0, 1.0, 0.0);
    let r = Ray::new(origin, direction);
    let m = translation(3.0, 4.0, 5.0);
    let r2 = r.transform(m);
    let expected_moved_origin = Tuple::point(4.0, 6.0, 8.0);
    assert_eq!(expected_moved_origin, r2.origin);
    assert_eq!(direction, r2.direction);
  }
  #[test]
  fn scaling_a_ray() {
    let origin = Tuple::point(1.0, 2.0, 3.0);
    let direction = Tuple::vector(0.0, 1.0, 0.0);
    let r = Ray::new(origin, direction);
    let m = scaling(2.0, 3.0, 4.0);
    let r2 = r.transform(m);
    let expected_moved_origin = Tuple::point(2.0, 6.0, 12.0);
    let expected_new_direction = Tuple::vector(0.0, 3.0, 0.0);
    assert_eq!(expected_moved_origin, r2.origin);
    assert_eq!(expected_new_direction, r2.direction);
  }

}
