pub mod plane;
pub mod worldstate;

pub trait BaseEntity {
    fn set_entity_state(&mut self, EntityState);
    fn get_entity_state(&self) -> &EntityState;
    fn get_mass(&self) -> f32;
    fn get_next_state(&self, f64) -> EntityState;
    fn update_state(&self, f64, f64);
    fn new_entity_with_state(&self, EntityState) -> Box<BaseEntity>;
    fn print(&self);
}

pub trait RoundedEntity {
    fn get_moment_inertia() -> f64;
}

#[derive(Debug)]
pub struct EntityState {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl EntityState {
    pub fn new(x: f32, y: f32, z: f32) -> EntityState {
        return EntityState { x: x, y: y, z: z };
    }
}

impl Clone for EntityState {
    fn clone(&self) -> EntityState {
        EntityState {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_creates_state() {
        let state = EntityState::new(1.0, 2.0, 3.0);
        assert_eq!(state.x, 1.0);
        assert_eq!(state.y, 2.0);
        assert_eq!(state.z, 3.0);
    }

    #[test]
    fn it_clones_state() {
        let state = EntityState::new(1.0, 2.0, 3.0);
        assert_eq!(state.x, 1.0);
        let mut cloned_state = state.clone();
        assert_eq!(state.x, cloned_state.x);
        assert_eq!(state.y, cloned_state.y);
        assert_eq!(state.z, cloned_state.z);
        cloned_state.x = 4.0;
        assert_ne!(state.x, cloned_state.x);
    }
}
