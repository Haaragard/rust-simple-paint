use std::{cell::RefCell, rc::Rc};

use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, pixels::Color, rect::{Point}, render::{Canvas, TextureCreator}, video::{Window, WindowContext}};

extern crate sdl2;

mod ui;

struct AppState {
    pub mouse_pressed: bool,
    pub mouse_position: Point,
    pub canvas: Rc<RefCell<Canvas<Window>>>,
}

impl AppState {
    pub fn new(canvas: Rc<RefCell<Canvas<Window>>>) -> Self {
        Self {
            mouse_pressed: false,
            mouse_position: Point::new(0, 0),
            canvas,
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

    let canvas = window.into_canvas()
        .build()
        .expect("failed to build a canvas");
    let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();

    let mut app_state = AppState::new(
        Rc::new(RefCell::new(canvas)),
    );

    {
        let mut canvas = app_state.canvas.borrow_mut();
        canvas.set_draw_color(Color::GREY);
        canvas.clear();
    }

    let scene1 = crate::ui::Scene {
        components: vec![
            Rc::new(RefCell::new(crate::ui::Button::new(&texture_creator, 50, 50, 120, 50))),
            Rc::new(RefCell::new(crate::ui::Button::new(&texture_creator, 150, 250, 120, 50))),
        ]
    };

    let renderer = crate::ui::Renderer::new(app_state.canvas.clone());

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::C), ..} => {
                    let mut canvas = app_state.canvas.borrow_mut();
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

        renderer.render(&scene1);

        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
