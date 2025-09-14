use std::{cell::RefCell, rc::Rc};

use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, pixels::Color, rect::{Point, Rect}, render::{Canvas, TextureCreator}, video::{Window, WindowContext}};

extern crate sdl2;

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

        // update(&app_state);

        renderer.render(&scene1);

        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}

// fn update(app_state: &AppState) {
//     draw_on_mouse_button_clicked(app_state);
// }

// fn draw_on_mouse_button_clicked(app_state: &AppState) {
//     let mut canvas = app_state.canvas.borrow_mut();

//     if app_state.mouse_pressed {
//         let center = Rect::from_center(
//             app_state.mouse_position.clone(),
//             10,
//             10
//         );

//         canvas.set_draw_color(Color::RGB(0, 0, 255));
//         let radius = 10;
//         let r2 = radius * radius;
//         for dy in -radius..=radius {
//             let dx = ((r2 - dy * dy) as f64).sqrt() as i32;
//             let y = center.y + dy;
//             let x1 = center.x - dx;
//             let x2 = center.x + dx;
//             canvas.draw_line(Point::new(x1, y), Point::new(x2, y)).unwrap();
//         }
//     }
// }

mod ui {
    use std::{cell::RefCell, rc::Rc};

    use sdl2::{pixels::{Color, PixelFormatEnum}, rect::{Rect}, render::{Canvas, Texture, TextureCreator}, video::{Window, WindowContext}};

    // #[derive(Clone, Copy)]
    // pub struct Input {
    //     pub mouse_pos: Point,
    //     pub mouse_down_edge: bool,
    //     pub mouse_up_edge: bool,
    //     pub mouse_held: bool,
    // }

    // impl Default for Input {
    //     fn default() -> Self {
    //         Self {
    //             mouse_pos: Point::new(0, 0),
    //             mouse_down_edge: false,
    //             mouse_up_edge: false,
    //             mouse_held: false,
    //         }
    //     }
    // }

    // pub enum UiEvent {
    //     ButtonClicked(Rc<RefCell<dyn Component>>),
    // }

    #[derive(Default)]
    pub struct ComponentProperty {
        pub x: i32,
        pub y: i32,

        pub height: u32,
        pub width: u32,
    }

    pub struct ComponentState {
        pub is_enabled: bool,
        pub is_dirty: bool,
    }

    impl Default for ComponentState {
        fn default() -> Self {
            Self {
                is_enabled: true,
                is_dirty: true,
            }
        }
    }

    pub trait Component {
        fn update(&mut self, canvas: &mut Canvas<Window>);
        fn render(&self, canvas: &mut Canvas<Window>);
    }

    // pub trait HoverComponent {
    //     fn on_hover(&self);
    // }

    // pub trait SelectComponent {
    //     fn on_select(&self);
    // }

    pub struct Button<'tc> {
        _texture_creator: Option<&'tc TextureCreator<WindowContext>>,
        pub prop: ComponentProperty,
        state: ComponentState,
        texture: Option<Texture<'tc>>,
        rect: Option<Rect>,
    }

    impl<'tc> Default for Button<'tc> {
        fn default() -> Self {
            Self {
                _texture_creator: None,
                prop: ComponentProperty::default(),
                state: ComponentState::default(),
                texture: None,
                rect: None,
            }
        }
    }

    impl<'tc> Component for Button<'tc> {
        fn update(&mut self, canvas: &mut Canvas<Window>) {
            self.build(canvas);
        }

        fn render(&self, canvas: &mut Canvas<Window>) {
            if let Some(texture) = &self.texture && let Some(dst) = self.rect {
                canvas.copy(texture, None, dst).unwrap();
            }
        }
    }

    impl<'tc> Button<'tc> {
        pub fn new(texture_creator: &'tc TextureCreator<WindowContext>, x: i32, y: i32, width: u32, height: u32) -> Self {
            Self {
                _texture_creator: Some(texture_creator),
                prop: ComponentProperty { x, y, height, width },
                ..Default::default()
            }
        }
        fn build(&mut self, canvas: &mut Canvas<Window>) {
            let texture_creator = self._texture_creator
                .expect("Texture creator must be setted");

            if self.rect.is_none() || self.state.is_dirty {
                self.rect = Some(Rect::new(
                    self.prop.x,
                    self.prop.y,
                    self.prop.width,
                    self.prop.height,
                ));
            }

            if self.texture.is_none() || self.state.is_dirty {
                let (w, h) = (self.prop.width, self.prop.height);

                let mut texture = texture_creator
                    .create_texture_target(PixelFormatEnum::RGBA8888, w, h)
                    .expect("failed to create button texture");

                canvas.with_texture_canvas(&mut texture, |tc| {
                    // fundo
                    tc.set_draw_color(Color::RGB(255, 0, 0));
                    tc.clear();

                    // borda
                    tc.set_draw_color(Color::RGB(0, 0, 0));
                    tc.draw_rect(Rect::new(0, 0, w, h)).unwrap();
                }).expect("failed to render button to texture");

                self.texture = Some(texture);
            }

            self.state.is_dirty = false;
        }
    }

    // #[derive(Default)]
    // pub struct Text {
    //     prop: ComponentProperty,
    //     state: ComponentState,
    // }

    // impl Component for Text {
    //     fn render(&self, _canvas: &mut Canvas<Window>) {
    //         // TODO
    //     }
    // }

    // #[derive(Default)]
    // pub struct Circle {
    //     prop: ComponentProperty,
    //     state: ComponentState,
    // }

    // impl Component for Circle {
    //     fn render(&self, _canvas: &mut Canvas<Window>) {
    //         // TODO
    //     }
    // }

    // struct Layout {
    //     // TODO
    // }

    pub struct Scene<'tc> {
        pub components: Vec<Rc<RefCell<dyn Component + 'tc>>>,
    }

    pub struct Renderer {
        canvas: Rc<RefCell<Canvas<Window>>>,
    }

    impl Renderer {
        pub fn new(canvas: Rc<RefCell<Canvas<Window>>>) -> Self {
            Self { canvas }
        }

        fn clear(&self) {
            let mut canvas = self.canvas.borrow_mut();
            canvas.set_draw_color(Color::GREY);
            canvas.clear();
        }

        pub fn render(
            &self,
            scene: &Scene,
        ) {
            self.clear();

            let mut canvas = self.canvas.borrow_mut();

            for component_ref in scene.components.iter() {
                let mut c = component_ref.borrow_mut();
                c.update(&mut canvas);
                c.render(&mut canvas);
            }

            canvas.present();
        }
    }
}
