use entity;
use util::math;
use util::vector3::Vector3;

#[derive(Debug)]
pub struct Sphere {
    name: String,
    position: Vector3,
    mass: f32,
    radius: f32,
    velocity: Vector3,
}

impl Sphere {
    pub fn new(name: String, position: Vector3, mass: f32, radius: f32, velocity: Vector3) -> Sphere {
        return Sphere {
            name: name,
            position: position,
            mass: mass,
            radius: radius,
            velocity: velocity,
        };
    }
}

impl Clone for Sphere {
    fn clone(&self) -> Sphere {
        Sphere {
            name: self.name.clone(),
            position: self.position,
            mass: self.mass,
            radius: self.radius,
            velocity: self.velocity,
        }
    }
}

impl entity::BaseEntity for Sphere {
    fn set_position(&mut self, position: Vector3) {
        self.position = position;
    }

    fn get_position(&self) -> &Vector3 {
        &self.position
    }

    fn get_mass(&self) -> f32 {
        return self.mass;
    }

    fn get_next_position(&self, dt: f32) -> Vector3 {
        return math::new_pos(self.position, self.get_next_velocity(dt), dt);
    }

    fn new_entity_with_state(&self, entity: Vector3) -> Box<entity::BaseEntity> {
        let sphere = self.clone();
        Box::new(sphere)
    }

    fn update_state(&self, t: f32, dt: f32) -> Box<entity::BaseEntity> {
        let mut sphere = self.clone();
        sphere.velocity = sphere.get_next_velocity(dt);
        sphere.position = sphere.get_next_position(dt);
        Box::new(sphere)
    }

    fn print(&self) {
        println!("{:?}", self);
    }

    fn get_net_acceleration(&self) -> Vector3 {
        return Vector3::new(0.0, math::gravity(self.mass), 0.0);
    }

    fn get_next_velocity(&self, dt: f32) -> Vector3 {
        let net_accel = self.get_net_acceleration();
        return math::velocity_from_acc(self.velocity, net_accel, dt);
    }
}

impl Sphere {
    pub fn is_collided(&self, other: &Sphere) -> bool {
        return math::detect_collide_sphere_to_sphere(
            self.position,
            other.position,
            self.radius,
            other.radius,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use entity::sphere::Sphere;
    #[test]
    fn it_is_collided() {
        let vec = Vector3::new(1.0, 1.0, 1.0);
        let sphere1 = Sphere::new("Sphere1".to_string(), vec, 1.0, 1.0, vec);
        let sphere2 = Sphere::new("Sphere2".to_string(), vec, 1.0, 1.0, vec);
        assert_eq!(true, sphere1.is_collided(&sphere2));
    }
}
