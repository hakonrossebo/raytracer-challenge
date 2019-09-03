use crate::intersections::*;
use crate::materials::Material;
use crate::matrix::Matrix;
use crate::rays::Ray;
use crate::tuple::Tuple;

#[derive(Debug, PartialEq)]
pub struct Sphere {
  origin: Tuple,
  radius: f64,
  pub transform: Matrix,
  pub material: Material,
}

impl Sphere {
  pub fn new() -> Sphere {
    Sphere {
      origin: Tuple::point(0.0, 0.0, 0.0),
      radius: 1.0,
      transform: Matrix::identity(),
      material: Material::new(),
    }
  }
  pub fn intersect(&self, in_ray: Ray) -> Vec<Intersection> {
    let ray = in_ray.transform(self.transform.clone().inverse());
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
      let i1 = Intersection::new(t1, self);
      let i2 = Intersection::new(t2, self);
      v = intersections(vec![i1, i2]);
    }
    v
  }

  pub fn set_transform(&mut self, t: Matrix) {
    self.transform = t;
  }

  pub fn normal_at(&self, world_point: Tuple) -> Tuple {
    // (p - Tuple::point(0.0, 0.0, 0.0)).normalize()
    let object_point = self.transform.inverse() * world_point;
    let object_normal = object_point - self.origin;
    let mut world_normal = self.transform.inverse().transpose() * object_normal;
    world_normal.set_w(0.0);
    world_normal.normalize()
  }

  pub fn set_material(&mut self, m: Material) {
    self.material = m;
  }
}

#[cfg(test)]

mod tests {
  use super::*;
  use crate::materials::Material;
  use crate::matrix::Matrix;
  use crate::transformations::{rotation_z, scaling, translation};
  use crate::tuple::Tuple;

  #[test]
  fn a_ray_intersects_a_sphere_at_two_points() {
    let origin = Tuple::point(0.0, 0.0, -5.0);
    let direction = Tuple::vector(0.0, 0.0, 1.0);
    let r = Ray::new(origin, direction);
    let s = Sphere::new();
    let xs = s.intersect(r);
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
    let xs = s.intersect(r);
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
    let xs = s.intersect(r);
    assert_eq!(0, xs.len());
  }
  #[test]
  fn a_ray_originates_inside_a_sphere() {
    let origin = Tuple::point(0.0, 0.0, 0.0);
    let direction = Tuple::vector(0.0, 0.0, 1.0);
    let r = Ray::new(origin, direction);
    let s = Sphere::new();
    let xs = s.intersect(r);
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
    let xs = s.intersect(r);
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
  #[test]
  fn intersecting_a_scaled_sphere_with_a_ray() {
    let origin = Tuple::point(0.0, 0.0, -5.0);
    let direction = Tuple::vector(0.0, 0.0, 1.0);
    let r = Ray::new(origin, direction);
    let mut s = Sphere::new();
    s.set_transform(scaling(2.0, 2.0, 2.0));
    let xs = s.intersect(r);
    assert_eq!(2, xs.len());
    assert_eq!(3.0, xs[0].t);
    assert_eq!(7.0, xs[1].t);
  }
  #[test]
  fn intersecting_a_translated_sphere_with_a_ray() {
    let origin = Tuple::point(0.0, 0.0, -5.0);
    let direction = Tuple::vector(0.0, 0.0, 1.0);
    let r = Ray::new(origin, direction);
    let mut s = Sphere::new();
    s.set_transform(translation(5.0, 0.0, 0.0));
    let xs = s.intersect(r);
    assert_eq!(0, xs.len());
  }
  #[test]
  fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
    let s = Sphere::new();
    let p = Tuple::point(1.0, 0.0, 0.0);
    let n = s.normal_at(p);
    let expected = Tuple::vector(1.0, 0.0, 0.0);
    assert_eq!(expected, n);
  }
  #[test]
  fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
    let s = Sphere::new();
    let p = Tuple::point(0.0, 1.0, 0.0);
    let n = s.normal_at(p);
    let expected = Tuple::vector(0.0, 1.0, 0.0);
    assert_eq!(expected, n);
  }
  #[test]
  fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
    let s = Sphere::new();
    let p = Tuple::point(0.0, 0.0, 1.0);
    let n = s.normal_at(p);
    let expected = Tuple::vector(0.0, 0.0, 1.0);
    assert_eq!(expected, n);
  }
  #[test]
  fn the_normal_on_a_sphere_at_a_nonazial_point() {
    let s = Sphere::new();
    let p = Tuple::point(3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0);
    let n = s.normal_at(p);
    let expected = Tuple::vector(3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0);
    assert_eq!(expected, n);
  }
  #[test]
  fn the_normal_is_a_normalized_vector() {
    let s = Sphere::new();
    let p = Tuple::point(3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0);
    let n = s.normal_at(p);
    let expected = n.normalize();
    assert_eq!(expected, n);
  }
  #[test]
  fn computing_the_normal_on_a_transalted_sphere() {
    let mut s = Sphere::new();
    s.set_transform(translation(0.0, 1.0, 0.0));

    let p = Tuple::point(0.0, 1.70711, -0.70711);
    let n = s.normal_at(p);
    let expected = Tuple::vector(0.0, 0.70711, -0.70711);
    assert_eq!(expected, n);
  }
  #[test]
  fn computing_the_normal_on_a_transformed_sphere() {
    let mut s = Sphere::new();
    let m = scaling(1.0, 0.5, 1.0) * rotation_z(std::f64::consts::PI / 5.0);
    s.set_transform(m);

    let p = Tuple::point(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
    let n = s.normal_at(p);
    let expected = Tuple::vector(0.0, 0.97014, -0.24254);
    assert_eq!(expected, n);
  }
  #[test]
  fn a_sphere_has_a_default_material() {
    let s = Sphere::new();
    let expected = Material::new();
    let m = s.material;
    assert_eq!(expected, m);
  }
  #[test]
  fn a_sphere_may_be_assigned_a_material() {
    let mut s = Sphere::new();
    let mut m = s.material;
    m.ambient = 1.0;
    s.set_material(m.clone());
    assert_eq!(m, s.material);
  }
}
