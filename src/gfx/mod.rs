extern crate three;
use std::collections::HashMap;
use entity::Entity;

pub struct Renderer {
    window: three::Window,
    camera: three::camera::Camera,
    entities: HashMap<String, Entity>,
}

impl Renderer {
    pub fn new() -> Renderer {
        let mut window = three::Window::new("Gentle AP Physics RS");
        window.scene.background = three::Background::Color(0xC6F0FF);

        let center = [0.0, 0.0];
        let yextent = 20.0;
        let zrange = -20.0 .. 20.0;
        let camera = window.factory.orthographic_camera(center, yextent, zrange);

        Renderer {
            window: window,
            camera: camera,
            entities: HashMap::<String, Entity>::new(),
        }
    }

    pub fn render(&mut self) -> bool {
        let is_updated = self.window.update();
        if is_updated {
            for (key, ent) in self.entities {
                ent.render(&self.window);
            }
            self.window.render(&self.camera);
        }
        is_updated
    }
}
