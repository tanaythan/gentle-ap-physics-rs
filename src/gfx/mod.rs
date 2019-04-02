extern crate three;
extern crate rayon_hash;
use self::rayon_hash::hash_map::HashMap;
use entity::Entity;

pub struct Renderer {
    window: three::Window,
    camera: three::camera::Camera,
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
        }
    }

    pub fn set_graphics(&mut self, entity: &mut Entity) {
        entity.set_graphics(&mut self.window);
    }

    pub fn render(&mut self, entities: &HashMap<String, Entity>) -> bool {
        let is_updated = self.window.update();
        if is_updated {
            for (_key, ent) in entities {
                ent.render();
            }
            self.window.render(&self.camera);
        }
        is_updated
    }
}
