// standard
use std::time::Duration;

// sdl2
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::render::{ WindowCanvas, Texture };
use sdl2::rect::{ Point, Rect };
use sdl2::image::{self, LoadTexture, InitFlag };

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture, // Pass texture by reference
    position: Point,
    sprite: Rect,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    // Treat the center of the screen as the (0, 0) coordinate
    let screen_position = position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, sprite.width(), sprite.height());
    canvas.copy(texture, sprite, screen_rect)?;

    canvas.present();

    Ok(())
}



pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    // Leading "_" tells Rust that this is an unused variable that we don't care about. It has to
    // stay unused because if we don't have any variable at all then Rust will treat it as a
    // temporary value and drop it right away!
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

    let window = video_subsystem.window("Hack 'n' Slash", 800, 600)
        .resizable()
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/bardo.png").unwrap();

    let position = Point::new(0, 0);
    // src position in the spritesheet
    let sprite = Rect::new(0, 0, 26, 36);


    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        // Update Background Color
        i = (i + 1) % 255;

        // Render
        render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture, position, sprite).unwrap();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

