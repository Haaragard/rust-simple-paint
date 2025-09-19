use std::{cell::RefCell, collections::HashMap, rc::Rc};

use sdl2::{pixels::{Color, PixelFormatEnum}, rect::{Point, Rect}, render::{Canvas, Texture, TextureCreator}, video::{Window, WindowContext}};

use crate::app::Input;

pub struct TextureManager<'tc> {
    texture_creator: &'tc TextureCreator<WindowContext>,
    cache: HashMap<String, Texture<'tc>>,
}

impl<'tc> TextureManager<'tc> {
    pub fn new(texture_creator: &'tc TextureCreator<WindowContext>) -> Self {
        Self {
            texture_creator,
            cache: HashMap::new(),
        }
    }

    pub fn get_texture(&mut self, key: &str) -> Option<&Texture<'tc>> {
        self.cache.get(key)
    }

    pub fn create_texture(&mut self, key: &str, width: u32, height: u32, canvas: &mut Canvas<Window>) -> &Texture<'tc> {
        let mut texture = self.texture_creator
            .create_texture_target(PixelFormatEnum::RGBA8888, width, height)
            .expect("failed to create texture");

        canvas.with_texture_canvas(&mut texture, |tc| {
            tc.set_draw_color(Color::RGB(255, 0, 0));
            tc.clear();
            tc.set_draw_color(Color::RGB(0, 0, 0));
            tc.draw_rect(Rect::new(0, 0, width, height)).unwrap();
        }).expect("failed to render");

        self.cache.insert(key.to_string(), texture);
        self.cache.get(key).unwrap()
    }

    pub fn drop_texture(&mut self, key: &str) {
        self.cache.remove(key);
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ComponentProperty {
    pub x: i32,
    pub y: i32,
    pub height: u32,
    pub width: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct ComponentState {
    pub is_enabled: bool,
    pub is_dirty: bool,
    pub is_focused: bool,
}

impl Default for ComponentState {
    fn default() -> Self {
        Self {
            is_enabled: true,
            is_dirty: true,
            is_focused: false,
        }
    }
}

pub trait Component {
    fn update(&mut self, input: Input);
    fn get_id(&self) -> &str;
    fn get_props(&self) -> ComponentProperty;
    fn get_state(&self) -> ComponentState;
    fn get_bounds(&self) -> Rect;
}

#[derive(Debug)]
pub struct Button {
    id: String,
    state: ComponentState,
    prop: ComponentProperty,
    boundary: Rect,
}

impl Default for Button {
    fn default() -> Self {
        let id = format!("BUTTON-{}", ulid::Ulid::new());
        let state = ComponentState::default();
        let prop = ComponentProperty::default();
        let boundary = Rect::new(
            prop.x,
            prop.y,
            prop.width,
            prop.height
        );

        Self { id, state, prop, boundary }
    }
}

impl Component for Button {
    fn update(&mut self, input: Input) {
        // self.state.is_dirty = false;

        if input.mouse_up {
            self.state.is_focused = false;
        } else if input.mouse_down && self.boundary.contains_point(input.mouse_pos) {
            self.state.is_focused = true;
        } else if self.state.is_focused {
            self.boundary.reposition(input.mouse_pos);

            self.prop.x = self.boundary.x;
            self.prop.y = self.boundary.y;

            self.state.is_dirty = true;
        }

        dbg!(&self);
    }

    fn get_id(&self) -> &str {
        self.id.as_str()
    }

    fn get_props(&self) -> ComponentProperty {
        self.prop
    }

    fn get_state(&self) -> ComponentState {
        self.state
    }

    fn get_bounds(&self) -> Rect {
        self.boundary
    }
}

impl Button {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        let prop = ComponentProperty { x, y, height, width };
        let boundary = Rect::new(x, y, width, height);

        Self {
            prop,
            boundary,
            ..Default::default()
        }
    }
}

pub struct Scene {
    pub components: Vec<Rc<RefCell<dyn Component>>>,
}

pub struct Renderer<'tc> {
    canvas: Rc<RefCell<Canvas<Window>>>,
    texture_manager: TextureManager<'tc>,
}

impl<'tc> Renderer<'tc> {
    pub fn new(canvas: Rc<RefCell<Canvas<Window>>>, texture_manager: TextureManager<'tc>) -> Self {
        Self {
            canvas,
            texture_manager,
        }
    }

    fn clear(&mut self) {
        let mut canvas = self.canvas.borrow_mut();
        canvas.set_draw_color(Color::GREY);
        canvas.clear();
    }

    pub fn render(&mut self, scene: &Scene, input: Input) {
        self.clear();

        let mut canvas = self.canvas.borrow_mut();
        for component_ref in scene.components.iter() {
            let mut c = component_ref.borrow_mut();
            c.update(input);

            let component_state = c.get_state();
            if component_state.is_enabled {
                let component_id = c.get_id();
                if component_state.is_dirty {
                    self.texture_manager.drop_texture(component_id);
                }

                let component_property = c.get_props();
                let texture = match self.texture_manager.get_texture(component_id) {
                    Some(texture) => texture,
                    None => self.texture_manager.create_texture(
                        component_id,
                        component_property.width,
                        component_property.height,
                        &mut canvas
                    )
                };

                canvas.copy(texture, None, c.get_bounds())
                    .expect(format!("Could not render texture for component ID: {}", component_id).as_str());
            }
        }

        canvas.present();
    }
}
