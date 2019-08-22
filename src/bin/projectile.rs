extern crate chrono;
extern crate raytracer_challenge;

use raytracer_challenge::canvas::Canvas;
use raytracer_challenge::tuple::Tuple;
use std::fs::File;
use std::io::Write;

use chrono::DateTime;
use chrono::Utc;

fn main() {
  let mut iterations: u64 = 0;
  let mut p = Projectile {
    position: Tuple::point(0.0, 1.0, 0.0),
    velocity: Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25,
  };
  let e = Environment {
    gravity: Tuple::vector(0.0, -0.1, 0.0),
    wind: Tuple::vector(-0.01, 0.0, 0.0),
  };
  let mut canvas = Canvas::new(900, 550);
  let red = Tuple::color(1.0, 0.0, 0.1);
  println!("Starting projectile...");
  while p.position.1 > 0.0 && iterations < 10 {
    iterations += 1;
    p = tick(&e, &p);
    println!(
      "Iterating...tick {}, x:{:.2}, y: {:.2}",
      iterations, p.position.0, p.position.1
    );
    let px = p.position.0.round() as usize;
    let mut pytmp = p.position.1.round() as i64;
    if pytmp <= 0 {
      pytmp = 1
    };
    let py = canvas.height - pytmp as usize;
    canvas.write_pixel(px, py, red);
  }
  println!("Writing canvas to ppm.");
  let ppm = canvas.canvas_to_ppm();
  println!("Writing ppm to file.");
  write_ppm_to_file(&ppm);
  println!("Finished.");
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
  Projectile {
    position: proj.position + proj.velocity,
    velocity: proj.velocity + env.gravity + env.wind,
  }
}

fn write_ppm_to_file(ppm: &String) {
  let date: DateTime<Utc> = Utc::now();
  let dateformatted = date.format("%Y-%m-%d_%H_%M_%S").to_string();
  let filename = format!("c:/Temp/render_test_{}.ppm", dateformatted);
  let mut file = File::create(filename).expect("Create file failed.");
  file
    .write_all(ppm.as_bytes())
    .expect("Writing file failed.");
  println!("File written to disk.")
}

#[derive(Debug)]
struct Projectile {
  position: Tuple,
  velocity: Tuple,
}

struct Environment {
  gravity: Tuple,
  wind: Tuple,
}
