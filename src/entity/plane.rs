use entity;

#[derive(Debug)]
pub struct Plane {
  state: entity::EntityState,
  mass: f32,
  width: f32,
  length: f32,
}

impl Plane {
  pub fn new(state: entity::EntityState, mass: f32, width: f32, length: f32) -> Plane {
    return Plane {
      state: state,
      mass: mass,
      width: width,
      length: length,
    };
  }
}
impl Clone for Plane {
  fn clone(&self) -> Plane {
    Plane {
      state: self.state.clone(),
      mass: self.mass,
      width: self.width,
      length: self.length,
    }
  }
}

impl entity::BaseEntity for Plane {
  fn set_entity_state(&mut self, state: entity::EntityState) {
    self.state = state.clone();
  }

  fn get_entity_state(&self) -> &entity::EntityState {
    &self.state
  }

  fn get_mass(&self) -> f32 {
    return self.mass;
  }

  fn get_next_state(&self, x: f64) -> entity::EntityState {
    self.state
  }

  fn new_entity_with_state(&self, entity: entity::EntityState) -> Box<entity::BaseEntity> {
    let plane = self.clone();
    Box::new(plane)
  }

  fn update_state(&self, val: f64, val2: f64) {}

  fn print(&self) {}
}

#[cfg(test)]
mod tests {
  use super::*;
  use entity::BaseEntity;
  #[test]
  fn it_creates_a_plane() {
    let state = entity::EntityState::new(3.0, 3.0, 4.0);
    let plane = Plane::new(state.clone(), 5.0, 1.0, 2.0);
    let ent_state = plane.get_entity_state();
    assert_eq!(ent_state.x, state.x);
    assert_eq!(ent_state.y, state.y);
    assert_eq!(ent_state.z, state.z);
  }
}
