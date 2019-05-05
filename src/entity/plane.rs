use entity;
use entity::three::Object;
use entity::BaseEntity;
use entity::Entity;
use std::collections::HashSet;
use util::math;
use util::vector3::Vector3;

#[derive(Debug, Clone)]
pub struct Plane {
    name: String,
    position: Vector3,
    mass: f32,
    width: f32,
    length: f32,
    mesh: Option<three::Mesh>,
    collision: HashSet<String>,
}

impl Plane {
    pub fn new(name: String, position: Vector3, mass: f32, width: f32, length: f32) -> Plane {
        Plane {
            name: name,
            position: position,
            mass: mass,
            width: width,
            length: length,
            mesh: None,
            collision: HashSet::new(),
        }
    }

    pub fn set_graphics(&mut self, window: &mut three::Window) {
        let mesh = {
            let geometry = three::Geometry::plane(self.width, self.length);
            let material = three::material::Wireframe { color: 0x0000FF };
            window.factory.mesh(geometry, material)
        };
        window.scene.add(&mesh);
        self.mesh = Some(mesh);
    }

    pub fn get_min_point(&self) -> Vector3 {
        Vector3::new(
            self.position.x - (self.width / 2.0),
            self.position.y,
            self.position.z - (self.length / 2.0),
        )
    }

    pub fn get_max_point(&self) -> Vector3 {
        Vector3::new(
            self.position.x + (self.width / 2.0),
            self.position.y,
            self.position.z + (self.length / 2.0),
        )
    }

    pub fn is_collided(&mut self, other: &Entity) -> bool {
        let res = match other {
            Entity::Plane(_) => false,
            Entity::Sphere(sphere) => math::detect_collide_sphere_to_plane(
                *sphere.get_position(),
                sphere.get_radius(),
                self.get_min_point(),
                self.get_max_point(),
            ),
        };

        if self.collision.contains(&other.get_name()) {
            if res {
                return false;
            } else {
                self.collision.remove(&other.get_name());
            }
        } else if res {
            self.collision.insert(other.get_name());
        }
        res
    }

    pub fn is_colliding(&self, other: &Entity) -> bool {
        self.collision.contains(&other.get_name())
    }

    pub fn render(&self) {
        match self.mesh {
            Some(ref mesh) => {
                mesh.set_position([self.position.x, self.position.y, self.position.z])
            }
            None => panic!("Can't render w/o graphics!"),
        }
    }
}

impl entity::BaseEntity for Plane {
    fn set_position(&mut self, position: Vector3) {
        self.position = position.clone();
    }

    fn get_position(&self) -> &Vector3 {
        &self.position
    }

    fn get_mass(&self) -> f32 {
        self.mass
    }

    fn get_next_position(&self, _dt: f32) -> Vector3 {
        self.position
    }

    fn new_entity_with_state(&self, _entity: Vector3) -> Entity {
        let plane = self.clone();
        Entity::Plane(plane)
    }

    fn update_state(&self, _t: f32, _dt: f32) -> Entity {
        Entity::Plane(self.clone())
    }

    fn print(&self) {
        println!("PLANE: ");
        println!("\t Name: {:?}", self.name);
        println!("\t Position: {:?}", self.position);
        println!("\t Mass: {:?}", self.mass);
        println!("\t Width: {:?}", self.width);
        println!("\t Length: {:?}", self.length);
    }

    fn get_net_acceleration(&self) -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }

    fn get_next_velocity(&self, _dt: f32) -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }

    fn apply_force(&mut self, _f: Vector3) {
        return;
    }

    fn collide_with_entity(&mut self, _other: Entity) {
        return;
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use entity::BaseEntity;
    #[test]
    fn it_creates_a_plane() {
        let position = Vector3::new(3.0, 3.0, 4.0);
        let plane = Plane::new("Plane1".to_string(), position, 5.0, 1.0, 2.0);
        let ent_state = plane.get_position();
        assert_eq!(ent_state.x, position.x);
        assert_eq!(ent_state.y, position.y);
        assert_eq!(ent_state.z, position.z);
    }
}
