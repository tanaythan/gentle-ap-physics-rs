pub mod plane;
pub mod sphere;
pub mod worldstate;
extern crate three;
use util::vector3::Vector3;

pub trait BaseEntity {
    fn set_position(&mut self, Vector3);
    fn get_position(&self) -> &Vector3;
    fn get_mass(&self) -> f32;
    fn get_next_position(&self, f32) -> Vector3;
    fn update_state(&self, f32, f32) -> Entity;
    fn new_entity_with_state(&self, Vector3) -> Entity;
    fn print(&self);
    fn get_net_acceleration(&self) -> Vector3;
    fn get_next_velocity(&self, dt: f32) -> Vector3;
    fn apply_force(&mut self, Vector3);
    fn collide_with_entity(&mut self, other: Entity);
    fn get_name(&self) -> String;
}

#[derive(Clone)]
pub enum Entity {
    Plane(plane::Plane),
    Sphere(sphere::Sphere),
}

impl Entity {
    pub fn render(&self) {
        match self {
            Entity::Plane(plane) => plane.render(),
            Entity::Sphere(sphere) => sphere.render(),
        }
    }

    pub fn set_graphics(&mut self, window: &mut three::Window) {
        match self {
            Entity::Plane(plane) => plane.set_graphics(window),
            Entity::Sphere(sphere) => sphere.set_graphics(window),
        }
    }

    pub fn is_collided(&mut self, ent: &Entity) -> bool {
        match self {
            Entity::Plane(plane) => plane.is_collided(ent),
            Entity::Sphere(sphere) => sphere.is_collided(ent),
        }
    }

    pub fn is_colliding(&self, ent: &Entity) -> bool {
        match self {
            Entity::Plane(plane) => plane.is_colliding(ent),
            Entity::Sphere(sphere) => sphere.is_colliding(ent),
        }
    }
}

impl BaseEntity for Entity {
    fn set_position(&mut self, vec: Vector3) {
        match self {
            Entity::Plane(plane) => plane.set_position(vec),
            Entity::Sphere(sphere) => sphere.set_position(vec),
        }
    }

    fn get_position(&self) -> &Vector3 {
        match self {
            Entity::Plane(plane) => plane.get_position(),
            Entity::Sphere(sphere) => sphere.get_position(),
        }
    }

    fn get_mass(&self) -> f32 {
        match self {
            Entity::Plane(plane) => plane.get_mass(),
            Entity::Sphere(sphere) => sphere.get_mass(),
        }
    }

    fn get_next_position(&self, dt: f32) -> Vector3 {
        match self {
            Entity::Plane(plane) => plane.get_next_position(dt),
            Entity::Sphere(sphere) => sphere.get_next_position(dt),
        }
    }

    fn update_state(&self, t: f32, dt: f32) -> Entity {
        match self {
            Entity::Plane(plane) => plane.update_state(t, dt),
            Entity::Sphere(sphere) => sphere.update_state(t, dt),
        }
    }

    fn new_entity_with_state(&self, vec: Vector3) -> Entity {
        match self {
            Entity::Plane(plane) => plane.new_entity_with_state(vec),
            Entity::Sphere(sphere) => sphere.new_entity_with_state(vec),
        }
    }

    fn print(&self) {
        match self {
            Entity::Plane(plane) => plane.print(),
            Entity::Sphere(sphere) => sphere.print(),
        }
    }

    fn get_net_acceleration(&self) -> Vector3 {
        match self {
            Entity::Plane(plane) => plane.get_net_acceleration(),
            Entity::Sphere(sphere) => sphere.get_net_acceleration(),
        }
    }

    fn get_next_velocity(&self, dt: f32) -> Vector3 {
        match self {
            Entity::Plane(plane) => plane.get_next_velocity(dt),
            Entity::Sphere(sphere) => sphere.get_next_velocity(dt),
        }
    }

    fn apply_force(&mut self, f: Vector3) {
        match self {
            Entity::Plane(plane) => plane.apply_force(f),
            Entity::Sphere(sphere) => sphere.apply_force(f),
        }
    }

    fn collide_with_entity(&mut self, other: Entity) {
        match self {
            Entity::Plane(plane) => plane.collide_with_entity(other),
            Entity::Sphere(sphere) => sphere.collide_with_entity(other),
        }
    }

    fn get_name(&self) -> String {
        match self {
            Entity::Plane(plane) => plane.get_name(),
            Entity::Sphere(sphere) => sphere.get_name(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_creates_state() {
        let state = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(state.x, 1.0);
        assert_eq!(state.y, 2.0);
        assert_eq!(state.z, 3.0);
    }

    #[test]
    fn it_clones_state() {
        let state = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(state.x, 1.0);
        let mut cloned_state = state.clone();
        assert_eq!(state.x, cloned_state.x);
        assert_eq!(state.y, cloned_state.y);
        assert_eq!(state.z, cloned_state.z);
        cloned_state.x = 4.0;
        assert_ne!(state.x, cloned_state.x);
    }
}
