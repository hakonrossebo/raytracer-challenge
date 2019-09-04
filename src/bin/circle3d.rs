extern crate chrono;
extern crate raytracer_challenge;

use raytracer_challenge::canvas::Canvas;
use raytracer_challenge::file::*;
use raytracer_challenge::intersections::hit;
use raytracer_challenge::lights::PointLight;
use raytracer_challenge::materials::Material;
use raytracer_challenge::rays::Ray;
use raytracer_challenge::spheres::Sphere;
use raytracer_challenge::tuple::Tuple;

fn main() {
  let canvas_width = 100;
  let canvas_height = 100;
  let ray_origin = Tuple::point(0.0, 0.0, -5.0);
  let wall_z = 10.0;
  let wall_size = 7.0;
  let pixel_size = wall_size / canvas_width as f64;
  let half = wall_size / 2.0;
  let mut shape = Sphere::new();
  let mut mat = Material::new();
  mat.color = Tuple::color(1.0, 0.2, 1.0);
  shape.set_material(mat);
  let light_position = Tuple::point(-10.0, 10.0, -10.0);
  let light_color = Tuple::color(1.0, 1.0, 1.0);
  let light = PointLight::new(light_position, light_color);

  // shape.set_transform(scaling(0.5, 1.0, 1.0));
  // shape.set_transform(shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * scaling(0.5, 1.0, 1.0));
  let mut canvas = Canvas::new(canvas_width, canvas_height);
  let mut color: Tuple;
  println!("Starting circle...");
  for y in 0..canvas_height {
    let world_y = half - pixel_size * y as f64;
    println!("Processing line...{} of {}", y, canvas_height);
    for x in 0..canvas_width {
      let world_x = -half + pixel_size * x as f64;
      let position = Tuple::point(world_x, world_y, wall_z);
      let r = Ray::new(ray_origin, (position - ray_origin).normalize());
      let xs = shape.intersect(r.clone());

      if let Some(hit) = hit(xs) {
        let point = r.clone().position(hit.t);
        let normal = hit.object.normal_at(point);
        let eye = -r.direction;
        color = hit
          .object
          .material
          .lighting(light.clone(), point, eye, normal);
        canvas.write_pixel(x, y, color);
      }
    }
  }
  // Done - writing file
  println!("Writing canvas to ppm.");
  let ppm = canvas.canvas_to_ppm();
  println!("Writing ppm to file.");
  write_ppm_to_file(&ppm, "Circle3d_test");
  println!("Finished.");
}
