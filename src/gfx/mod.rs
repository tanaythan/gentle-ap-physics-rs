extern crate three;
use std::collections::HashMap;
use entity::Entity;

// Add renderable types here
enum Renderable {
    Fake,
}

impl Renderable {
    pub fn new() -> Renderable {
        Renderable::Fake
    }
}

pub struct Renderer {
    window: three::Window,
    camera: three::camera::Camera,
    renderables: HashMap<String, Renderable>,
}

impl Renderer {
    pub fn new() -> Renderer {
        let mut window = three::Window::new("Gentle AP Physics RS");
        window.scene.background = three::Background::Color(0xC6F0FF);

        let center = [0.0, 0.0];
        let yextent = 1.0;
        let zrange = -1.0 .. 1.0;
        let camera = window.factory.orthographic_camera(center, yextent, zrange);

        Renderer {
            window: window,
            camera: camera,
            renderables: HashMap::<String, Renderable>::new(),
        }
    }

    pub fn update_ent(&mut self, entities: HashMap<String, Entity>) {
        for (key, ent) in entities {
            if !self.renderables.contains_key(&key) {
                self.renderables.insert(key.clone(), Renderable::new());
            }
            // Update entity position here
        }
    }

    pub fn render(&mut self) -> bool {
        let is_updated = self.window.update();
        if is_updated {
            self.window.render(&self.camera);
        }
        is_updated
    }
}
