pub mod plane;
pub mod sphere;
pub mod worldstate;

use std::ops;

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
}

impl<T> BaseEntityClone for T
where
    T: 'static + BaseEntity + Clone,
{
    fn clone_box(&self) -> Box<BaseEntity> {
        Box::new(self.clone())
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

#[derive(Debug, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        return Vector3 { x: x, y: y, z: z };
    }
}

impl Clone for Vector3 {
    fn clone(&self) -> Vector3 {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, _rhs: Vector3) -> Vector3 {
        return Vector3::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z);
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, _rhs: f32) -> Vector3 {
        return Vector3::new(self.x * _rhs, self.y * _rhs, self.z * _rhs);
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
