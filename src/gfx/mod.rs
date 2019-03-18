use entity::plane::Plane;

pub trait Renderable {
    fn setup() -> three::Object;
    fn render();
}

impl Renderable for Plane {
    fn setup() -> three::Object {

    }
    fn render() {
        
    }
}
