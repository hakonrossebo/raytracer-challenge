use crate::matrix::Matrix;
use crate::tuple::Tuple;

pub struct Ray {
  origin: Tuple,
  direction: Tuple,
}

impl Ray {
  pub fn new(o: Tuple, d: Tuple) -> Ray {
    Ray {
      origin: o,
      direction: d,
    }
  }
}

#[cfg(test)]

mod tests {
  use super::*;
  use crate::tuple::Tuple;
  #[test]
  fn creating_and_querying_a_ray() {
    let origin = Tuple::point(1.0, 2.0, 3.0);
    let direction = Tuple::vector(4.0, 5.0, 6.0);
    let r = Ray::new(origin, direction);
    assert_eq!(origin, r.origin);
    assert_eq!(direction, r.direction);
  }

}
