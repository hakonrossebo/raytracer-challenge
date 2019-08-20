// extern crate raytracer_challenge;

// use raytracer_challenge::tuple::Tuple;
use crate::tuple::Tuple;

struct Canvas {
    pub width: usize,
    pub height: usize,
    pub canvas: Vec<Tuple>,
}

impl Canvas {
    pub fn new(w: usize, h: usize) -> Canvas {
        let c = vec![Tuple::color(0.0, 0.0, 0.0); w * h];
        Canvas {width: w, height: h, canvas: c}
    }
    pub fn write_pixel(&mut self, x: usize, y: usize, color: Tuple) {
        let pos = self.width * y + x;
        self.canvas[pos] = color;
    }
    pub fn pixel_at(self, x: usize, y: usize) -> Tuple {
        let pos = self.width * y + x;
        self.canvas[pos]
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

}