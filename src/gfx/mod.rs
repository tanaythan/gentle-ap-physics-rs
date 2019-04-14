extern crate three;
extern crate rayon_hash;
use self::rayon_hash::hash_map::HashMap;
use entity::Entity;
use self::three::Object;

pub struct Renderer {
    window: three::Window,
    camera: three::camera::Camera,
}

impl Renderer {
    pub fn new() -> Renderer {
        let mut window = three::Window::new("Gentle AP Physics RS");
        window.scene.background = three::Background::Color(0x000000);

        let yrange = 100.0;
        let zdepth = 1.0 .. 20.0;
        let camera = window.factory.perspective_camera(yrange, zdepth);
        //camera.set_position([0.0, -8.0, 20.0]);
        //camera.set_orientation([0.707, 0.0, 0.707, 0.0]);
        camera.look_at([0.0, 0.0, 20.0], [0.0, 1.5, 0.0], None);
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
