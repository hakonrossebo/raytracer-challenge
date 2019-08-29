use crate::matrix::Matrix;
use crate::rays::Ray;
use crate::tuple::Tuple;

#[derive(Debug, PartialEq)]
pub struct Sphere {
  origin: Tuple,
  radius: f64,
}

impl Sphere {
  pub fn new() -> Sphere {
    Sphere {
      origin: Tuple::point(0.0, 0.0, 0.0),
      radius: 1.0,
    }
  }
  pub fn intersects(&self, ray: Ray) -> Vec<f64> {
    let mut v: Vec<f64> = Vec::new();

    let sphere_to_ray = ray.origin - self.origin;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * ray.direction.dot(sphere_to_ray);
    let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
    let discriminant: f64 = b * b - 4.0 * a * c;
    if discriminant >= 0.0 {
      let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
      let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
      v.push(t1);
      v.push(t2);
    }
    v
  }
}

#[cfg(test)]

mod tests {
  use super::*;
  use crate::tuple::Tuple;

  #[test]
  fn a_ray_intersects_a_sphere_at_two_points() {
    let origin = Tuple::point(0.0, 0.0, -5.0);
    let direction = Tuple::vector(0.0, 0.0, 1.0);
    let r = Ray::new(origin, direction);
    let s = Sphere::new();
    let xs = s.intersects(r);
    assert_eq!(2, xs.len());
    assert_eq!(4.0, xs[0]);
    assert_eq!(6.0, xs[1]);
  }
  #[test]
  fn a_ray_intersects_a_sphere_at_a_tangent() {
    let origin = Tuple::point(0.0, 1.0, -5.0);
    let direction = Tuple::vector(0.0, 0.0, 1.0);
    let r = Ray::new(origin, direction);
    let s = Sphere::new();
    let xs = s.intersects(r);
    assert_eq!(2, xs.len());
    assert_eq!(5.0, xs[0]);
    assert_eq!(5.0, xs[1]);
  }
  #[test]
  fn a_ray_intersects_misses_a_sphere() {
    let origin = Tuple::point(0.0, 2.0, -5.0);
    let direction = Tuple::vector(0.0, 0.0, 1.0);
    let r = Ray::new(origin, direction);
    let s = Sphere::new();
    let xs = s.intersects(r);
    assert_eq!(0, xs.len());
  }
  #[test]
  fn a_ray_originates_inside_a_sphere() {
    let origin = Tuple::point(0.0, 0.0, 0.0);
    let direction = Tuple::vector(0.0, 0.0, 1.0);
    let r = Ray::new(origin, direction);
    let s = Sphere::new();
    let xs = s.intersects(r);
    assert_eq!(2, xs.len());
    assert_eq!(-1.0, xs[0]);
    assert_eq!(1.0, xs[1]);
  }
  #[test]
  fn a_sphere_is_behind_a_ray() {
    let origin = Tuple::point(0.0, 0.0, 5.0);
    let direction = Tuple::vector(0.0, 0.0, 1.0);
    let r = Ray::new(origin, direction);
    let s = Sphere::new();
    let xs = s.intersects(r);
    assert_eq!(2, xs.len());
    assert_eq!(-6.0, xs[0]);
    assert_eq!(-4.0, xs[1]);
  }

}
