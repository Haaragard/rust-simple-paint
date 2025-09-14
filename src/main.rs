use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, pixels::Color, rect::{Point, Rect}, render::Canvas, video::Window};

extern crate sdl2;

struct AppState {
    pub mouse_pressed: bool,
    pub mouse_position: Point,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            mouse_pressed: false,
            mouse_position: Point::new(0, 0),
        }
    }
}

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

    let mut app_state = AppState::default();

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
                Event::KeyDown { keycode: Some(Keycode::C), ..} => {
                    canvas.set_draw_color(Color::GRAY);
                    canvas.clear();
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => {
                    app_state.mouse_pressed = true;
                },
                Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => {
                    app_state.mouse_pressed = false;
                },
                Event::MouseMotion { x, y, .. } => {
                    app_state.mouse_position.x = x;
                    app_state.mouse_position.y = y;
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here

        draw_on_mouse_button_clicked(&app_state, &mut canvas);

        canvas.present();

        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn draw_on_mouse_button_clicked(app_state: &AppState, canvas: &mut Canvas<Window>) {
    if app_state.mouse_pressed {
        let center = Rect::from_center(
            app_state.mouse_position.clone(),
            10,
            10
        );

        canvas.set_draw_color(Color::RGB(0, 0, 255));
        let radius = 10;
        let r2 = radius * radius;
        for dy in -radius..=radius {
            let dx = ((r2 - dy * dy) as f64).sqrt() as i32;
            let y = center.y + dy;
            let x1 = center.x - dx;
            let x2 = center.x + dx;
            canvas.draw_line(Point::new(x1, y), Point::new(x2, y)).unwrap();
        }
    }
}
