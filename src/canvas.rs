use crate::tuple::Tuple;
const COLORSCALE: i32 = 255;

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
        if x >= 0 && x <= self.width && y >= 0 && y <= self.height {
            let pos = self.width * y + x;
            self.canvas[pos] = color;
        }
    }
    pub fn pixel_at(&self, x: usize, y: usize) -> Tuple {
        let pos = self.width * y + x;
        self.canvas[pos]
    }

    pub fn canvas_to_ppm(&self) -> String {
        let header = format!("P3\n{} {}\n{}\n", self.width, self.height, COLORSCALE);

        // helper function to convert rgb values
        fn convert_and_clamp_color(color: f64) -> i32 {
            let converted_color = (color * 256.0) as i32;
            if converted_color > COLORSCALE {
                COLORSCALE
            } else if converted_color < 0 {
                0
            } else {
                converted_color
            }
        }
        // helper function to add colors to line according to ppm rules
        fn colors_to_ppm_string(v: &[i32]) -> String {
            let mut pos = 0;
            let mut s = String::new();
            for i in v {
                let n = format!("{}", i);
                if pos + n.len() >= 68 {
                    s.push('\n');
                    pos = 0;
                }
                if pos != 0 {
                    s.push(' ');
                    pos += 1;
                }
                s.push_str(&n);
                pos += n.len();
            }
            s
        }
        // generating the ppm info from canvas
        let mut ppm = String::from(header);
        for y in 0..self.height {
            // let mut line = String::new();
            let mut linecolors: Vec<i32> = Vec::new();
            for x in 0..self.width {
                let Tuple(r, g, b, _) = self.pixel_at(x, y);
                linecolors.push(convert_and_clamp_color(r));
                linecolors.push(convert_and_clamp_color(g));
                linecolors.push(convert_and_clamp_color(b));
                // push_color_to_line(&mut line, r);
                // push_color_to_line(&mut line, g);
                // push_color_to_line(&mut line, b);
            }
            ppm.push_str(&colors_to_ppm_string(&linecolors));
            ppm.push('\n');
        }
        ppm.push('\n');
        ppm
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
            .take(4)
            .collect::<Vec<&str>>()
            .join("\n");
        let expected = format!("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n");
        assert_eq!(expected, ppm);
    }
    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);
        let color = Tuple::color(1.0, 0.8, 0.6);
        for i in 0..10 {
            for j in 0..2 {
                c.write_pixel(i, j, color);
            }
        }
        let ppm = c
            .canvas_to_ppm()
            .lines()
            .skip(3)
            .take(5)
            .collect::<Vec<&str>>()
            .join("\n");
        let expected = format!("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n");
        assert_eq!(expected, ppm);
    }

}
