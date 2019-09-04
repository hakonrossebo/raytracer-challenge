extern crate chrono;
extern crate raytracer_challenge;

use raytracer_challenge::file::*;
use raytracer_challenge::canvas::Canvas;
use raytracer_challenge::intersections::hit;
use raytracer_challenge::rays::Ray;
use raytracer_challenge::spheres::Sphere;
use raytracer_challenge::transformations::*;
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
  // shape.set_transform(scaling(0.5, 1.0, 1.0));
  shape.set_transform(shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * scaling(0.5, 1.0, 1.0));
  let mut canvas = Canvas::new(canvas_width, canvas_height);
  let color = Tuple::color(1.0, 0.0, 0.0);
  println!("Starting circle...");
  for y in 0..canvas_height {
    let world_y = half - pixel_size * y as f64;
    for x in 0..canvas_width {
      let world_x = half - pixel_size * x as f64;
      let position = Tuple::point(world_x, world_y, wall_z);
      let r = Ray::new(ray_origin, (position - ray_origin).normalize());
      let xs = shape.intersect(r);

      if let Some(_) = hit(xs) {
        canvas.write_pixel(x, canvas_height - y, color);
      }
    }
  }
  // Done - writing file
  println!("Writing canvas to ppm.");
  let ppm = canvas.canvas_to_ppm();
  println!("Writing ppm to file.");
  write_ppm_to_file(&ppm, "Circle_test");
  println!("Finished.");
}
