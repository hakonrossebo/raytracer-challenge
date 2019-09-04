extern crate image as im;
extern crate piston_window;
use piston_window::*;

fn main() {
    let opengl = OpenGL::V3_2;
    let (width, height) = (100, 100);

    let mut canvas = im::ImageBuffer::new(width, height);

    let mut window: PistonWindow =
        WindowSettings::new("Raytracer challenge in Rust", (width, height))
            .exit_on_esc(true)
            .graphics_api(opengl)
            .build()
            .unwrap();
    window.set_lazy(true);
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };
    let mut texture: G2dTexture =
        Texture::from_image(&mut texture_context, &canvas, &TextureSettings::new()).unwrap();

    window.set_lazy(true);
    canvas.put_pixel(22, 22, im::Rgba([0, 0, 0, 255]));
    while let Some(e) = window.next() {
        canvas.put_pixel(42, 42, im::Rgba([0, 0, 0, 255]));
        texture.update(&mut texture_context, &canvas).unwrap();
        window.draw_2d(&e, |c, g, device| {
            // Update texture before rendering.
            texture_context.encoder.flush(device);

            clear([1.0; 4], g);
            image(&texture, c.transform, g);
        });
    }
}
