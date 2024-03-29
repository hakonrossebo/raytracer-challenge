extern crate chrono;
extern crate raytracer_challenge;

extern crate image as im;
extern crate piston_window;
use piston_window::*;
use raytracer_challenge::canvas::Canvas;
use raytracer_challenge::file::*;
use raytracer_challenge::intersections::hit;
use raytracer_challenge::lights::PointLight;
use raytracer_challenge::materials::Material;
use raytracer_challenge::rays::Ray;
use raytracer_challenge::spheres::Sphere;
use raytracer_challenge::tuple::Tuple;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

#[derive(Debug, Clone)]
pub struct Pixel {
    pub x: u32,
    pub y: u32,
    pub c: Tuple,
}

fn main() {
    let (width, height) = (100, 100);
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        perform_render(width, height, &tx);
    });

    let opengl = OpenGL::V3_2;

    let mut canvas = im::ImageBuffer::new(width, height);

    let mut window: PistonWindow =
        WindowSettings::new("Raytracer challenge in Rust", (width, height))
            .exit_on_esc(true)
            .graphics_api(opengl)
            .build()
            .unwrap();
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };
    let mut texture: G2dTexture =
        Texture::from_image(&mut texture_context, &canvas, &TextureSettings::new()).unwrap();

    while let Some(e) = window.next() {
        let received = rx.try_recv();
        if let Ok(newlinepixels) = received {
            for newpx in newlinepixels.iter() {
                canvas.put_pixel(
                    newpx.x as u32,
                    newpx.y as u32,
                    im::Rgba([
                        convert_and_clamp(newpx.c.0),
                        convert_and_clamp(newpx.c.1),
                        convert_and_clamp(newpx.c.2),
                        255,
                    ]),
                );
            }
        }
        texture.update(&mut texture_context, &canvas).unwrap();
        window.draw_2d(&e, |c, g, device| {
            // Update texture before rendering.
            texture_context.encoder.flush(device);

            clear([0.0; 4], g);
            image(&texture, c.transform, g);
        });
    }
}

fn convert_and_clamp(colval: f64) -> u8 {
    let mut res = (colval * 255.0) as u16;
    if res > 255 {
        res = 255;
    }
    res as u8
}

fn perform_render(canvas_width: u32, canvas_height: u32, s: &std::sync::mpsc::Sender<Vec<Pixel>>) {
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / f64::from(canvas_width);
    let half = wall_size / 2.0;
    let mut shape = Sphere::default();
    let mut mat = Material::default();
    mat.color = Tuple::color(1.0, 0.2, 1.0);
    shape.set_material(mat);
    let light_position = Tuple::point(-10.0, 10.0, -10.0);
    let light_color = Tuple::color(1.0, 1.0, 1.0);
    let light = PointLight::new(light_position, light_color);

    // shape.set_transform(scaling(0.5, 1.0, 1.0));
    // shape.set_transform(shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * scaling(0.5, 1.0, 1.0));
    let mut canvas = Canvas::new(canvas_width as usize, canvas_height as usize);
    let mut color: Tuple;
    println!("Starting circle...");
    for y in 0..canvas_height {
        let world_y = half - pixel_size * f64::from(y);
        println!("Processing line...{} of {}", y, canvas_height);
        let mut pixels: Vec<Pixel> = Vec::new();
        for x in 0..canvas_width {
            let world_x = -half + pixel_size * f64::from(x);
            let position = Tuple::point(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(r.clone());

            if let Some(hit) = hit(xs) {
                let point = r.clone().position(hit.t);
                let normal = hit.object.normal_at(point);
                let eye = -r.direction;
                let alight = Arc::new(light.clone());
                color = hit.object.material.lighting(&alight, point, eye, normal);
                canvas.write_pixel(x as usize, y as usize, color);
                pixels.push(Pixel { x, y, c: color });
            }
        }
        s.send(pixels).unwrap();
    }
    // Done - writing file
    println!("Writing canvas to ppm.");
    let ppm = canvas.canvas_to_ppm();
    println!("Writing ppm to file.");
    write_ppm_to_file(&ppm, "Circle3d_test");
    println!("Finished.");
}
