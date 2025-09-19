use std::{cell::RefCell, rc::Rc};

use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, rect::Point, render::{Canvas}, video::{Window}, Sdl, VideoSubsystem};

use crate::ui::{Renderer, TextureManager, UiEvent};

pub struct AppState {
    pub canvas: Rc<RefCell<Canvas<Window>>>,
    pub _ui_events: Rc<RefCell<Vec<UiEvent>>>,

    scenes: Rc<RefCell<Vec<crate::ui::Scene>>>,
    active_scene: u32,
}

impl AppState {
    pub fn new(canvas: Rc<RefCell<Canvas<Window>>>) -> Self {
        let ui_events = Rc::new(RefCell::new(Vec::<crate::ui::UiEvent>::new()));
        let scenes = Rc::new(RefCell::new(Vec::<crate::ui::Scene>::new()));

        Self {
            canvas,
            _ui_events: ui_events,

            scenes,
            active_scene: 0,
        }
    }

    pub fn add_scene(&mut self, scene: crate::ui::Scene) {
        self.scenes.borrow_mut().push(scene);
    }
}

pub struct App {
    context: Sdl,
    _video_subsystem: VideoSubsystem,
    app_state: AppState,
}

impl App {
    pub fn new() -> Self {
        let context = sdl2::init().expect("could not init sdl2 context.");
        let video_subsystem = context.video().expect("could not fetch video subsystem.");
        let window = video_subsystem.window("SDL2 Window", 800, 600)
            .position_centered()
            .build()
            .expect("could not init a window.");

        let canvas = window.into_canvas()
            .build()
            .expect("failed to build a canvas");

        let app_state = AppState::new(
            Rc::new(RefCell::new(canvas)),
        );

        Self {
            context,
            _video_subsystem: video_subsystem,
            app_state,
        }
    }

    pub fn add_scene(&mut self, scene: crate::ui::Scene) {
        self.app_state.add_scene(scene);
    }

    pub fn run(&self) {
        let texture_creator = self.app_state.canvas.borrow_mut().texture_creator();
        let mut renderer = Renderer::new(
            self.app_state.canvas.clone(),
            TextureManager::new(&texture_creator)
        );

        let mut event_pump = self.context.event_pump().unwrap();
        let mut input = Input::default();
        'running: loop {
            input.mouse_down = false;
            input.mouse_up = false;

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running;
                    },
                    Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => {
                        input.mouse_down = true;
                    },
                    Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => {
                        input.mouse_up = true;
                    },
                    Event::MouseMotion { x, y, .. } => {
                        input.mouse_pos.x = x;
                        input.mouse_pos.y = y;
                    },
                    _ => {}
                }
            }
            // The rest of the game loop goes here

            println!("Inputs before render:");
            dbg!(input);
            println!();

            let scenes = self.app_state.scenes.borrow();
            if let Some(scene) = scenes.get(self.app_state.active_scene as usize) {
                renderer.render(scene, input);
            }

            std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Input {
    pub mouse_pos: Point,
    pub mouse_down: bool,
    pub mouse_up: bool,
    pub mouse_held: bool,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            mouse_pos: Point::new(0, 0),
            mouse_down: false,
            mouse_up: false,
            mouse_held: false,
        }
    }
}
