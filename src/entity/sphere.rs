use entity;
use entity::Entity;
use util::math;
use util::vector3::Vector3;

#[derive(Debug, Clone)]
pub struct Sphere {
    name: String,
    position: Vector3,
    mass: f32,
    radius: f32,
    velocity: Vector3,
}

impl Sphere {
    pub fn new(
        name: String,
        position: Vector3,
        mass: f32,
        radius: f32,
        velocity: Vector3,
    ) -> Sphere {
        return Sphere {
            name: name,
            position: position,
            mass: mass,
            radius: radius,
            velocity: velocity,
        };
    }

    pub fn is_collided(&self, other: Entity) -> bool {
        match other {
            Entity::Sphere(sphere) => math::detect_collide_sphere_to_sphere(
                self.position,
                sphere.position,
                self.radius,
                sphere.radius,
            ),
            Entity::Plane(plane) => math::detect_collide_sphere_to_plane(
                self.position,
                self.radius,
                plane.get_min_point(),
                plane.get_max_point(),
            ),
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

    fn new_entity_with_state(&self, entity: Vector3) -> Entity {
        let sphere = self.clone();
        Entity::Sphere(sphere)
    }

    fn update_state(&self, t: f32, dt: f32) -> Entity {
        let mut sphere = self.clone();
        sphere.velocity = sphere.get_next_velocity(dt);
        sphere.position = sphere.get_next_position(dt);
        Entity::Sphere(sphere)
    }

    fn print(&self) {
        println!("{:?}", self);
    }

    fn get_net_acceleration(&self) -> Vector3 {
        return Vector3::new(0.0, -1.0 * math::gravity(self.mass), 0.0);
    }

    fn get_next_velocity(&self, dt: f32) -> Vector3 {
        let net_accel = self.get_net_acceleration();
        return math::velocity_from_acc(self.velocity, net_accel, dt);
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
        assert_eq!(true, sphere1.is_collided(Entity::Sphere(sphere2)));
    }

    #[test]
    fn it_plane_collisions() {
        let vec = Vector3::new(1.0, 1.0, 1.0);
        let sphere1 = Sphere::new("Sphere1".to_string(), vec, 1.0, 1.0, vec);
        let plane1 = entity::plane::Plane::new("Plane1".to_string(), vec, 1.0, 1.0, 1.0);
        assert_eq!(true, sphere1.is_collided(Entity::Plane(plane1)));
        let plane2 = entity::plane::Plane::new(
            "Plane2".to_string(),
            Vector3::new(4.0, 4.0, 4.0),
            1.0,
            1.0,
            1.0,
        );
        assert_eq!(false, sphere1.is_collided(Entity::Plane(plane2)));
    }
}
