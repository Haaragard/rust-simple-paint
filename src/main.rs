use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

extern crate sdl2;

fn main() {
    let sdl_context = sdl2::init().expect("could not init sdl2 context.");
    let video_subsystem = sdl_context.video().expect("could not fetch video subsystem.");

    let window = video_subsystem.window("SDL2 Window", 800, 600)
        .position_centered()
        .build()
        .expect("could not init a window.");

    let mut canvas = window.into_canvas()
            .build()
            .expect("failed to build a canvas");

    canvas.set_draw_color(Color::GREY);
    canvas.clear();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here

        canvas.present();

        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
