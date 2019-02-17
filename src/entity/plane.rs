use entity;
use entity::BaseEntity;
use entity::Entity;
use util::math;
use util::vector3::Vector3;

#[derive(Debug, Clone)]
pub struct Plane {
    name: String,
    position: Vector3,
    mass: f32,
    width: f32,
    length: f32,
}

impl Plane {
    pub fn new(name: String, position: Vector3, mass: f32, width: f32, length: f32) -> Plane {
        return Plane {
            name: name,
            position: position,
            mass: mass,
            width: width,
            length: length,
        };
    }

    pub fn get_min_point(&self) -> Vector3 {
        return Vector3::new(
            self.position.x - (self.width / 2.0),
            self.position.y,
            self.position.z - (self.length / 2.0),
        );
    }

    pub fn get_max_point(&self) -> Vector3 {
        return Vector3::new(
            self.position.x + (self.width / 2.0),
            self.position.y,
            self.position.z + (self.length / 2.0),
        );
    }

    pub fn is_collided(&self, ent: &Entity) -> bool {
        match ent {
            Entity::Plane(_) => false,
            Entity::Sphere(sphere) => math::detect_collide_sphere_to_plane(
                *sphere.get_position(),
                sphere.get_radius(),
                self.get_min_point(),
                self.get_max_point(),
            ),
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
        return self.mass;
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
        println!("{:?}", self);
    }

    fn get_net_acceleration(&self) -> Vector3 {
        return Vector3::new(0.0, 0.0, 0.0);
    }

    fn get_next_velocity(&self, _dt: f32) -> Vector3 {
        return Vector3::new(0.0, 0.0, 0.0);
    }

    fn apply_force(&mut self, _f: Vector3) {
        return;
    }

    fn collide_with_entity(&mut self, _other: Entity) {
        return;
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
