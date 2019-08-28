extern crate chrono;
extern crate raytracer_challenge;

use raytracer_challenge::canvas::Canvas;
use raytracer_challenge::transformations;
use raytracer_challenge::tuple::Tuple;
use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;

use chrono::DateTime;
use chrono::Utc;

fn main() {
  let canvas_width = 550;
  let canvas_height = 550;
  let radius = 3.0 / 8.0 * (canvas_width as f64);
  let mut canvas = Canvas::new(canvas_width, canvas_height);
  let color = Tuple::color(1.0, 0.0, 0.0);
  let twelve = Tuple::point(0.0, 0.0, 1.0);

  println!("Starting clock...");
  for time in 0..12 {
    let r = transformations::rotation_y(time as f64 * PI / 6.0);
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
  write_ppm_to_file(&ppm);
  println!("Finished.");
}

fn write_ppm_to_file(ppm: &String) {
  let date: DateTime<Utc> = Utc::now();
  let dateformatted = date.format("%Y-%m-%d_%H_%M_%S").to_string();
  let filename = format!("c:/Temp/clock_test_{}.ppm", dateformatted);
  let mut file = File::create(filename).expect("Create file failed.");
  file
    .write_all(ppm.as_bytes())
    .expect("Writing file failed.");
  println!("File written to disk.")
}
