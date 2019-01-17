pub mod plane;
pub mod sphere;
pub mod worldstate;
use util::vector3::Vector3;
use std::any::Any;

pub trait BaseEntity: BaseEntityClone {
    fn set_position(&mut self, Vector3);
    fn get_position(&self) -> &Vector3;
    fn get_mass(&self) -> f32;
    fn get_next_position(&self, f32) -> Vector3;
    fn update_state(&self, f32, f32) -> Box<BaseEntity>;
    fn new_entity_with_state(&self, Vector3) -> Box<BaseEntity>;
    fn print(&self);
    fn get_net_acceleration(&self) -> Vector3;
    fn get_next_velocity(&self, dt: f32) -> Vector3;
}

pub trait BaseEntityClone {
    fn clone_box(&self) -> Box<BaseEntity>;
    fn as_any(&self) -> &dyn Any;
}

impl<T> BaseEntityClone for T
where
    T: 'static + BaseEntity + Clone,
{
    fn clone_box(&self) -> Box<BaseEntity> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Clone for Box<BaseEntity> {
    fn clone(&self) -> Box<BaseEntity> {
        self.clone_box()
    }
}

pub trait RoundedEntity {
    fn get_moment_inertia() -> f64;
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
