extern crate chrono;
extern crate raytracer_challenge;

use raytracer_challenge::canvas::Canvas;
use raytracer_challenge::file::write_ppm_to_file;
use raytracer_challenge::tuple::Tuple;

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
  let mut mag;
  let mut color;
  //  = Tuple::color(1.0, 0.0, 1.0 - mag);
  println!("Starting projectile...");
  while p.position.1 > 0.0 && iterations < 1000 {
    iterations += 1;
    mag = 1.0 / p.velocity.magnitude() * 4.0;
    color = Tuple::color(1.0, 0.0 + mag, 1.0 - mag);
    println!(
      "Iterating...tick {}, x:{:.2}, y: {:.2}",
      iterations, p.position.0, p.position.1
    );
    let px = p.position.0.round() as usize;
    let py = canvas.height - (p.position.1.round() as usize);
    canvas.write_pixel(px, py, color);
    p = tick(&e, &p);
  }
  println!("Writing canvas to ppm.");
  let ppm = canvas.canvas_to_ppm();
  println!("Writing ppm to file.");
  write_ppm_to_file(&ppm, "Projectile_test");
  println!("Finished.");
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
  Projectile {
    position: proj.position + proj.velocity,
    velocity: proj.velocity + env.gravity + env.wind,
  }
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
