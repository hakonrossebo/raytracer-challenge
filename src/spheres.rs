use crate::intersections::*;
use crate::matrix::Matrix;
use crate::rays::Ray;
use crate::tuple::Tuple;

#[derive(Debug, PartialEq)]
pub struct Sphere {
  origin: Tuple,
  radius: f64,
  pub transform: Matrix,
}

impl Sphere {
  pub fn new() -> Sphere {
    Sphere {
      origin: Tuple::point(0.0, 0.0, 0.0),
      radius: 1.0,
      transform: Matrix::identity(),
    }
  }
  pub fn intersects(&self, ray: Ray) -> Vec<Intersection> {
    let mut v: Vec<Intersection> = Vec::new();

    let sphere_to_ray = ray.origin - self.origin;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * ray.direction.dot(sphere_to_ray);
    let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
    let discriminant: f64 = b * b - 4.0 * a * c;
    let t1;
    let t2;
    if discriminant >= 0.0 {
      t1 = (-b - discriminant.sqrt()) / (2.0 * a);
      t2 = (-b + discriminant.sqrt()) / (2.0 * a);
      // let i1 = Intersection::new(t1, self);
      // let i2 = Intersection::new(t2, self);
      // v.push(Intersection::new(t1, &self));
      // v.push(Intersection::new(t2, &self));
      v = vec![Intersection::new(t1, self), Intersection::new(t2, self)]
    }
    // intersections(v)
    v
  }

  pub fn set_transform(&mut self, t: Matrix) {
    self.transform = t;
  }
}

#[cfg(test)]

mod tests {
  use super::*;
  use crate::matrix::Matrix;
  use crate::transformations::translation;
  use crate::tuple::Tuple;

  #[test]
  fn a_ray_intersects_a_sphere_at_two_points() {
    let origin = Tuple::point(0.0, 0.0, -5.0);
    let direction = Tuple::vector(0.0, 0.0, 1.0);
    let r = Ray::new(origin, direction);
    let s = Sphere::new();
    let xs = s.intersects(r);
    assert_eq!(2, xs.len());
    assert_eq!(4.0, xs[0].t);
    assert_eq!(6.0, xs[1].t);
  }
  #[test]
  fn a_ray_intersects_a_sphere_at_a_tangent() {
    let origin = Tuple::point(0.0, 1.0, -5.0);
    let direction = Tuple::vector(0.0, 0.0, 1.0);
    let r = Ray::new(origin, direction);
    let s = Sphere::new();
    let xs = s.intersects(r);
    assert_eq!(2, xs.len());
    assert_eq!(5.0, xs[0].t);
    assert_eq!(5.0, xs[1].t);
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
    assert_eq!(-1.0, xs[0].t);
    assert_eq!(1.0, xs[1].t);
  }
  #[test]
  fn a_sphere_is_behind_a_ray() {
    let origin = Tuple::point(0.0, 0.0, 5.0);
    let direction = Tuple::vector(0.0, 0.0, 1.0);
    let r = Ray::new(origin, direction);
    let s = Sphere::new();
    let xs = s.intersects(r);
    assert_eq!(2, xs.len());
    assert_eq!(-6.0, xs[0].t);
    assert_eq!(-4.0, xs[1].t);
  }
  #[test]
  fn a_spheres_default_transformation() {
    let s = Sphere::new();
    assert_eq!(Matrix::identity(), s.transform);
  }
  #[test]
  fn changing_a_spheres_transformation() {
    let mut s = Sphere::new();
    let t = translation(2.0, 3.0, 4.0);
    //TODO: Can I avoid clone here?
    s.set_transform(t.clone());
    assert_eq!(t, s.transform);
  }

}
