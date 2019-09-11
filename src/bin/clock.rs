extern crate chrono;
extern crate raytracer_challenge;

use raytracer_challenge::canvas::Canvas;
use raytracer_challenge::file::write_ppm_to_file;
use raytracer_challenge::transformations;
use raytracer_challenge::tuple::Tuple;
use std::f64::consts::PI;

fn main() {
  let canvas_width = 550;
  let canvas_height = 550;
  let radius = 3.0 / 8.0 * (canvas_width as f64);
  let mut canvas = Canvas::new(canvas_width, canvas_height);
  let color = Tuple::color(1.0, 0.0, 0.0);
  let twelve = Tuple::point(0.0, 0.0, 1.0);

  println!("Starting clock...");
  for time in 0..12 {
    let r = transformations::rotation_y(f64::from(time) * PI / 6.0);
    let Tuple(timepos_x, _, timepos_y, _) = r * twelve;

    println!("Rotating clock...");
    let px = (timepos_x as f64 * radius + canvas_width as f64 / 2.0).round() as usize;
    let py = (timepos_y as f64 * radius + canvas_height as f64 / 2.0).round() as usize;
    println!("x:{}, y: {}", px, py);
    canvas.write_pixel(px, py, color);
  }
  println!("Writing canvas to ppm.");
  let ppm = canvas.canvas_to_ppm();
  println!("Writing ppm to file.");
  write_ppm_to_file(&ppm, "Clock_test");
  println!("Finished.");
}
