extern crate chrono;
extern crate num_cpus;
extern crate raytracer_challenge;

extern crate image as im;
extern crate piston_window;
use piston_window::*;
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
use threadpool::ThreadPool;

#[derive(Debug, Clone)]
pub struct Pixel {
    pub x: u32,
    pub y: u32,
    pub c: Tuple,
}

fn main() {
    let (width, height) = (200, 200);
    let (tx, rx) = mpsc::channel();

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
    thread::spawn(move || {
        perform_render(width, height, &tx);
    });

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
    println!("Finished - Saving image.");
    let filename = create_os_path("multithread_image", "png");
    canvas.save_with_format(filename, im::PNG).unwrap();
}

fn convert_and_clamp(colval: f64) -> u8 {
    let mut res = (colval * 255.0) as u16;
    if res > 255 {
        res = 255;
    }
    res as u8
}

fn perform_render(canvas_width: u32, canvas_height: u32, s: &std::sync::mpsc::Sender<Vec<Pixel>>) {
    let num_logical_cpus = num_cpus::get();
    let num_physical_cpus = num_cpus::get_physical();
    println!("Logical CPUs: {}", num_logical_cpus);
    println!("Physical CPUs: {}", num_physical_cpus);

    let n_workers = num_physical_cpus;
    let pool = ThreadPool::new(n_workers);

    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / canvas_width as f64;
    let half = wall_size / 2.0;
    let mut shape = Sphere::new();
    let mut mat = Material::new();
    mat.color = Tuple::color(1.0, 1.0, 0.2);
    shape.set_material(mat);
    let ashape = Arc::new(shape);
    let light_position = Tuple::point(-10.0, 10.0, -10.0);
    let light_color = Tuple::color(1.0, 1.0, 1.0);
    let light = Arc::new(PointLight::new(light_position, light_color));

    // shape.set_transform(scaling(0.5, 1.0, 1.0));
    // shape.set_transform(shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * scaling(0.5, 1.0, 1.0));
    println!("Starting circle...");
    for y in 0..canvas_height {
        let s = s.clone();
        // let shape = shape.clone();
        // let shape_clone:Arc<&Sphere> = Arc::clone(&shape);
        let shape_clone = Arc::clone(&ashape);
        let light_clone = Arc::clone(&light);

        pool.execute(move || {
            let world_y = half - pixel_size * y as f64;
            println!("Processing line...{} of {}", y, canvas_height);
            let mut pixels: Vec<Pixel> = Vec::new();
            for x in 0..canvas_width {
                let world_x = -half + pixel_size * x as f64;
                let position = Tuple::point(world_x, world_y, wall_z);
                let r = Ray::new(ray_origin, (position - ray_origin).normalize());
                let xs = shape_clone.intersect(r.clone());

                if let Some(hit) = hit(xs) {
                    let point = r.clone().position(hit.t);
                    let normal = hit.object.normal_at(point);
                    let eye = -r.direction;
                    let color = hit
                        .object
                        .material
                        .lighting(&light_clone, point, eye, normal);
                    pixels.push(Pixel {
                        x: x,
                        y: y,
                        c: color,
                    });
                }
            }
            s.send(pixels).unwrap();
        });
    }
    println!("Finished.");
}
