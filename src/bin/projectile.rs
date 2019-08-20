extern crate raytracer_challenge;

use raytracer_challenge::tuple::Tuple;

fn main() {
  let mut iterations: u64 = 0;
  let mut p = Projectile {
    position: Tuple::point(0.0, 1.0, 0.0),
    velocity: Tuple::vector(1.0, 1.0, 0.0).normalize() * 12.0,
  };
  let e = Environment {
    gravity: Tuple::vector(0.0, -0.1, 0.0),
    wind: Tuple::vector(-0.01, 0.0, 0.0),
  };
  println!("Starting projectile...");
  while p.position.1 > 0.0 && iterations < 1000 {
    iterations += 1;
    p = tick(&e, &p);
    println!(
      "Iterating...tick {}, x:{:.2}, y: {:.2}",
      iterations, p.position.0, p.position.1
    );
  }
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
