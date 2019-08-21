use crate::tuple::Tuple;

const COLORSCALE: usize = 255;

pub struct Canvas {
  pub width: usize,
  pub height: usize,
  pub canvas: Vec<Tuple>,
}

impl Canvas {
  pub fn new(w: usize, h: usize) -> Canvas {
    let c = vec![Tuple::color(0.0, 0.0, 0.0); w * h];
    Canvas {
      width: w,
      height: h,
      canvas: c,
    }
  }
  pub fn write_pixel(&mut self, x: usize, y: usize, color: Tuple) {
    let pos = self.width * y + x;
    self.canvas[pos] = color;
  }
  pub fn pixel_at(self, x: usize, y: usize) -> Tuple {
    let pos = self.width * y + x;
    self.canvas[pos]
  }

  pub fn canvas_to_ppm(&self) -> String {
    let header = format!("P3\n{} {}\n{}", self.width, self.height, COLORSCALE);
    fn clamp_color(color: usize) -> usize {
      0
    }
    header
  }
}

#[cfg(test)]

mod tests {
  use super::*;

  #[test]
  fn creating_a_canvas() {
    let c = Canvas::new(10, 20);
    let black = Tuple::color(0.0, 0.0, 0.0);
    assert_eq!(10, c.width);
    assert_eq!(20, c.height);
    for pixel in c.canvas {
      assert_eq!(black, pixel);
    }
  }
  #[test]
  fn writing_pixel_to_canvas() {
    let mut c = Canvas::new(10, 20);
    let red = Tuple::color(1.0, 0.0, 0.0);

    c.write_pixel(2, 3, red);
    let actual = c.pixel_at(2, 3);
    assert_eq!(red, actual);
  }
  #[test]
  fn constructing_the_ppm_header() {
    let c = Canvas::new(5, 3);
    let ppm = c
      .canvas_to_ppm()
      .lines()
      .take(3)
      .collect::<Vec<&str>>()
      .join("\n");
    let expected = format!("P3\n5 3\n{}", COLORSCALE);
    assert_eq!(expected, ppm);
  }
  #[test]
  fn constructing_the_ppm_pixel_data() {
    let mut c = Canvas::new(5, 3);
    let c1 = Tuple::color(1.5, 0.0, 0.0);
    let c2 = Tuple::color(0.0, 0.5, 0.0);
    let c3 = Tuple::color(-0.5, 0.0, 1.0);
    c.write_pixel(0, 0, c1);
    c.write_pixel(2, 1, c2);
    c.write_pixel(4, 2, c3);
    let ppm = c
      .canvas_to_ppm()
      .lines()
      .skip(3)
      .take(3)
      .collect::<Vec<&str>>()
      .join("\n");
    let expected = format!("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n");
    assert_eq!(expected, ppm);
  }

}
