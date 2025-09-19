use std::{cell::RefCell, rc::Rc};

use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, pixels::Color, rect::{Point}, render::{Canvas, TextureCreator}, video::{Window, WindowContext}};

use crate::app::App;

extern crate sdl2;

mod app;
mod ui;


fn main() {
    let mut app = App::new();

    let scene1 = {
        crate::ui::Scene {
            components: vec![
                Rc::new(RefCell::new(crate::ui::Button::new(50, 50, 120, 50))),
                Rc::new(RefCell::new(crate::ui::Button::new(150, 250, 120, 50))),
            ]
        }
    };

    app.add_scene(scene1);
    app.run();
}
