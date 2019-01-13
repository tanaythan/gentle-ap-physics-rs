use entity;
use util::vector3::Vector3;

#[derive(Debug)]
pub struct Plane {
    position: Vector3,
    mass: f32,
    width: f32,
    length: f32,
}

impl Plane {
    pub fn new(position: Vector3, mass: f32, width: f32, length: f32) -> Plane {
        return Plane {
            position: position,
            mass: mass,
            width: width,
            length: length,
        };
    }
}
impl Clone for Plane {
    fn clone(&self) -> Plane {
        Plane {
            position: self.position.clone(),
            mass: self.mass,
            width: self.width,
            length: self.length,
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

    fn get_next_position(&self, dt: f32) -> Vector3 {
        self.position.clone()
    }

    fn new_entity_with_state(&self, entity: Vector3) -> Box<entity::BaseEntity> {
        let plane = self.clone();
        Box::new(plane)
    }

    fn update_state(&self, t: f32, dt: f32) -> Box<entity::BaseEntity> {
        Box::new(self.clone())
    }

    fn print(&self) {
        println!("{:?}", self);
    }

    fn get_net_acceleration(&self) -> Vector3 {
        return Vector3::new(0.0, 0.0, 0.0);
    }

    fn get_next_velocity(&self, dt: f32) -> Vector3 {
        return Vector3::new(0.0, 0.0, 0.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use entity::BaseEntity;
    #[test]
    fn it_creates_a_plane() {
        let position = Vector3::new(3.0, 3.0, 4.0);
        let plane = Plane::new(position.clone(), 5.0, 1.0, 2.0);
        let ent_state = plane.get_position();
        assert_eq!(ent_state.x, position.x);
        assert_eq!(ent_state.y, position.y);
        assert_eq!(ent_state.z, position.z);
    }
}
