extern crate image as im;
extern crate piston_window;
use piston_window::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn testrender(val: u32, s: &std::sync::mpsc::Sender<u32>) {
    s.send(val).unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![22, 33, 44, 55, 66, 77, 88, 99];

        for val in vals {
            testrender(val, &tx);
            // tx.send(val).unwrap();
            // thread::sleep(Duration::from_secs(1));
        }
    });

    let opengl = OpenGL::V3_2;
    let (width, height) = (500, 500);

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

    // window.set_lazy(true);
    // canvas.put_pixel(22, 22, im::Rgba([0, 0, 0, 255]));
    while let Some(e) = window.next() {
        let received = rx.try_recv();
        if let Ok(newpx) = received {
            canvas.put_pixel(newpx, 42, im::Rgba([0, 0, 0, 255]));
        }
        texture.update(&mut texture_context, &canvas).unwrap();
        window.draw_2d(&e, |c, g, device| {
            // Update texture before rendering.
            texture_context.encoder.flush(device);

            clear([1.0; 4], g);
            image(&texture, c.transform, g);
        });
    }
}
